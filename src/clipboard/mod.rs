#[cfg(windows)]
pub mod windows_clipborad;
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub mod x11_clipboard;

pub trait ClipboardProvider {
    fn get_contents(&mut self) -> anyhow::Result<String>;
    fn set_contents(&mut self, contents: String) -> anyhow::Result<()>;
}
