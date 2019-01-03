
use std::io;

use log::info;
use tui::backend::CrosstermBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;
use winreg::RegKey;
use winreg::enums::*;

struct AdapterPairInfo {
    mac: String,
    devices: Vec<DevicePairInfo>,
}

struct DevicePairInfo {
}

fn main() -> Result<(), failure::Error> {

    env_logger::init();

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let key_7zip = hklm.open_subkey("SOFTWARE\\7-Zip").unwrap();
    let path: String = key_7zip.get_value("Path").unwrap();
    info!("{:?}", path);

    let dir_apple = hklm.open_subkey("SOFTWARE\\Apple Computer, Inc.").unwrap();
    for key in dir_apple.enum_keys() {
        info!("{:?}", key);
    }

    let mut terminal = Terminal::new(CrosstermBackend::new())?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let text = [
                Text::styled("It works!", Style::default().fg(Color::Green)),
            ];
            Paragraph::new(text.iter())
                .block(
                    Block::default()
                )
                .render(&mut f, size);
        })?;

        let input = crossterm::input();
        match input.read_char()? {
            'q' => break,
            _ => {}
        };
    }

    Ok(())
}
