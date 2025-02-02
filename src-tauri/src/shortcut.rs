use anyhow::{anyhow, Context, Result};
use mouse_position::mouse_position::Mouse;
use tauri::Window;

use selection::get_text;

/// 鼠标坐标与选中内容
///
/// Mouse coordinates and selected content
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShowVO {
    pub x: i32,
    pub y: i32,
    pub context: String,
}

pub fn show(panel: &Window) -> Result<()> {
    let context = get_text();

    if context.is_empty() {
        panel.emit("hide", ()).unwrap();
        return Ok(());
    }

    // 获取当前显示器的宽度和高度
    // Get the current monitor width and height
    let monitor = panel
        .current_monitor()
        .context("Failed to get current monitor")?
        .ok_or(anyhow!("Current monitor not found"))?;

    let monitor_size = monitor.size();
    let monitor_width = monitor_size.width as i32;
    let monitor_height = monitor_size.height as i32;

    // 获取panel的宽高
    // Get the width and height of the panel
    let panel_size = panel.outer_size()?;
    let panel_width = panel_size.width as i32;
    let panel_height = panel_size.height as i32;

    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { mut x, mut y } => {
            #[cfg(target_os = "macos")]
            {
                let scale_factor = monitor.scale_factor();
                x = (x as f64 * scale_factor) as i32;
                y = (y as f64 * scale_factor) as i32;
            }
            // 计算偏移量
            // Calculate the offset
            x -= 40;
            y += 20;

            // 边界检查
            // Boundary check
            if x < 0 {
                x = 0;
            }
            if x > monitor_width - panel_width {
                x = monitor_width - panel_width;
            }
            if y > monitor_height - panel_height {
                y = monitor_height - panel_height;
            }

            panel.emit("show", ShowVO { x, y, context }).unwrap();
        }
        Mouse::Error => println!("Error getting mouse position"),
    };
    Ok(())
}
