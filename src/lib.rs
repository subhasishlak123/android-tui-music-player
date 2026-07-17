use android_activity::AndroidApp;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge}, // Fixed: Added Block and Borders here
};
use std::io::stdout;

// --- THEME DEFINITION ---
struct Theme {
    accent: Color,
    bg: Color,
}
const NEON_CYBER: Theme = Theme { 
    accent: Color::Cyan, 
    bg: Color::Rgb(10, 10, 20) 
};

#[no_mangle]
fn android_main(app: AndroidApp) {
    // Initialize Terminal
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        terminal.draw(|f| {
            let area = f.size();
            
            // Background Block
            let main_block = Block::default()
                .title(" FUTURE TUI PLAYER ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(NEON_CYBER.accent))
                .bg(NEON_CYBER.bg);

            // A simple "Music Progress" Gauge
            let gauge = Gauge::default()
                .block(Block::default().title(" NOW PLAYING ").fg(NEON_CYBER.accent))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(45);

            // Render widgets
            f.render_widget(main_block, area);
            
            // Center the gauge slightly
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3), // Space for title
                    Constraint::Length(3), // The gauge
                    Constraint::Min(0),    // Remainder
                ])
                .split(area);
                
            f.render_widget(gauge, chunks[1]);
            
        }).unwrap();

        // Android apps need to handle events to prevent "Not Responding" errors
        // For a basic TUI, we just loop for now. 
        // In the future, add event polling here.
    }
}
