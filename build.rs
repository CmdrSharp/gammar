#[cfg(windows)]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/favicon.ico");

    res.compile().unwrap();
}
