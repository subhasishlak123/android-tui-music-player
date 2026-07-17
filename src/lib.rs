use android_activity::AndroidApp;
use ratatui::prelude::*;
use std::io::stdout;

#[no_mangle]
fn android_main(app: AndroidApp) {
    // 1. Initialize Terminal
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    // 2. Main Loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(" FUTURE TUI PLAYER ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .bg(Color::Rgb(10, 10, 20));
            
            f.render_widget(block, size);
        }).unwrap();

        // Check for Android events (touches, etc.) here
    }
}
