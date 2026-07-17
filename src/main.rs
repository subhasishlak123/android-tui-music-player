use ratatui::{prelude::*, widgets::*};
use rodio::{Decoder, OutputStream, Sink};
use std::io;

// --- 1. THEMES (The "Future Look") ---
struct Theme {
    accent: Color,
    bg: Color,
}
const NEON_CYBER: Theme = Theme { accent: Color::Magenta, bg: Color::Rgb(10, 10, 20) };

// --- 2. APP STATE ---
struct App {
    playing: bool,
    progress: f64,
}

// --- 3. UI RENDERING ---
fn ui(frame: &mut Frame, app: &App) {
    let area = frame.size();
    
    // Minimal layout: Visualizer on top, Info on bottom
    let chunks = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).split(area);

    // Future Look: A block with a neon border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(NEON_CYBER.accent))
        .title(" TUI PLAYER ");

    // Progress Bar (The music visual)
    let gauge = Gauge::default()
        .block(Block::default().title("Volume"))
        .gauge_style(Style::default().fg(NEON_CYBER.accent))
        .percent((app.progress * 100.0) as u16);

    frame.render_widget(block, chunks[0]);
    frame.render_widget(gauge, chunks[1]);
}

fn main() -> io::Result<()> {
    // Ratadroid handles the Terminal initialization here
    Ok(())
}
