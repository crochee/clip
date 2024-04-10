use super::ClipboardProvider;

use anyhow::Result;
use clipboard_win::{formats, get_clipboard, set_clipboard};

pub struct WindowsClipboardContext;

impl ClipboardProvider for WindowsClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        Ok(get_clipboard(formats::Unicode).map_err(|e| anyhow::anyhow!(e))?)
    }

    fn set_contents(&mut self, contents: String) -> Result<()> {
        Ok(set_clipboard(formats::Unicode, contents).map_err(|e| anyhow::anyhow!(e))?)
    }
}

impl WindowsClipboardContext {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}
