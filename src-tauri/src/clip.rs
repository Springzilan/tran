use anyhow::Result;
use arboard::Clipboard;

/// 写入剪贴板
///
/// Write to clipboard
pub fn set(context: String) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(context)?;
    Ok(())
}
