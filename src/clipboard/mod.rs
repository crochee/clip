#[cfg(windows)]
mod windows_clipborad;
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
mod x11_clipboard;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub type ClipboardContext = x11_clipboard::X11ClipboardContext<x11_clipboard::Clipboard>;
#[cfg(windows)]
pub type ClipboardContext = windows_clipborad::WindowsClipboardContext;

pub trait ClipboardProvider {
    fn get_contents(&mut self) -> anyhow::Result<String>;
    fn set_contents(&mut self, contents: String) -> anyhow::Result<()>;
}
