use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    sync::{Arc, Mutex},
};
use windows::core::{BOOL, PCWSTR};
use windows::Win32::Foundation::{GetLastError, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    CreateDCW, DeleteDC, EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFOEXW,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub id: String,
    pub name: String,
    pub device_name: String,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub gamma: f32,
    pub brightness: f32,
    pub contrast: f32,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            gamma: 1.0,
            brightness: 0.0,
            contrast: 1.0,
        }
    }
}

impl DisplaySettings {
    pub fn new(gamma: f32, brightness: f32, contrast: f32) -> Self {
        Self {
            gamma: gamma.clamp(0.1, 3.0),
            brightness: brightness.clamp(-1.0, 1.0),
            contrast: contrast.clamp(0.1, 3.0),
        }
    }
}

#[derive(Debug)]
pub struct GammaError(String);

impl fmt::Display for GammaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gamma control error: {}", self.0)
    }
}

impl Error for GammaError {}

pub fn enumerate_monitors() -> Vec<MonitorInfo> {
    let monitors: Arc<Mutex<Vec<MonitorInfo>>> = Arc::new(Mutex::new(Vec::new()));
    let monitors_clone = monitors.clone();

    extern "system" fn monitor_enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = unsafe { &*(lparam.0 as *const Arc<Mutex<Vec<MonitorInfo>>>) };

        let mut monitor_info = MONITORINFOEXW {
            monitorInfo: windows::Win32::Graphics::Gdi::MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFOEXW>() as u32,
                ..Default::default()
            },
            ..Default::default()
        };

        let info_result =
            unsafe { GetMonitorInfoW(hmonitor, &mut monitor_info.monitorInfo as *mut _ as *mut _) };

        if info_result.as_bool() {
            let device_name_end = monitor_info
                .szDevice
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(monitor_info.szDevice.len());

            let device_name = String::from_utf16_lossy(&monitor_info.szDevice[..device_name_end]);
            let is_primary = (monitor_info.monitorInfo.dwFlags & 1) != 0;

            monitors.lock().unwrap().push(MonitorInfo {
                id: format!("{:?}", hmonitor.0 as usize),
                name: if is_primary {
                    format!("{} (Primary)", device_name)
                } else {
                    device_name.clone()
                },
                device_name,
                is_primary,
            });
        }

        BOOL::from(true)
    }

    let _ = unsafe {
        EnumDisplayMonitors(
            None,
            None,
            Some(monitor_enum_proc),
            LPARAM(&monitors_clone as *const _ as isize),
        )
    };

    let mut result = monitors.lock().unwrap().clone();
    result.sort_by(|a, b| b.is_primary.cmp(&a.is_primary));

    result
}

pub fn apply_display_settings_to_monitor(
    settings: DisplaySettings,
    monitor: &MonitorInfo,
) -> Result<(), GammaError> {
    #[link(name = "gdi32")]
    extern "system" {
        fn SetDeviceGammaRamp(hdc: *mut std::ffi::c_void, lpRamp: *const u16) -> i32;
    }

    // Build gamma ramp array
    let mut ramp = [0u16; 768];

    for i in 0..256 {
        let value = ((((i as f32 / 255.0).powf(1.0 / settings.gamma) - 0.5) * settings.contrast
            + 0.5
            + settings.brightness)
            .clamp(0.0, 1.0)
            * 65535.0) as u16;

        ramp[i] = value;
        ramp[i + 256] = value;
        ramp[i + 512] = value;
    }

    // Convert device name to wide string
    let device_name_wide: Vec<u16> = monitor
        .device_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let hdc = unsafe {
        CreateDCW(
            PCWSTR(device_name_wide.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            None,
        )
    };

    if hdc.is_invalid() {
        return Err(GammaError(format!(
            "Failed to create DC for monitor: {}",
            monitor.name
        )));
    }

    let result = unsafe { SetDeviceGammaRamp(hdc.0, ramp.as_ptr()) };
    let _ = unsafe { DeleteDC(hdc) };

    if result != 0 {
        return Ok(());
    }

    let error_code = unsafe { GetLastError() };

    Err(GammaError(format!(
        "Failed to set gamma ramp for {}: {:?}",
        monitor.name, error_code
    )))
}
