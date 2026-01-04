# Gammar

This application allows you to control display settings on Windows using keybinds and profiles. It was created as an alternative to using the Nvidia Control Panel to adjust these settings.

- Control brightness, gamma & saturation with configurable keybinds
- Create profiles (and set keybinds for them) with preferred display settings

The application requires no installation; just download the executable and run it.

## Development

This application contains `unsafe` code blocks that directly interface with the Windows API to control display settings. Specifically:

- **Windows GDI API calls** - The application uses FFI to call `SetDeviceGammaRamp`, `GetMonitorInfoW`, `CreateDCW`, and `DeleteDC` functions from the Windows Graphics Device Interface (GDI).
- **Raw pointer manipulation** - Monitor enumeration callbacks require passing raw pointers to data structures, which necessitates unsafe dereferencing.
- **Device context** - Direct manipulation of device contexts (HDC) for each monitor to apply gamma correction.

These unsafe operations are necessary to achieve low-level control over display hardware that isn't exposed through safe APIs. I've limited them in scope as much as possible.

## License

[MIT](https://choosealicense.com/licenses/mit/)
