use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent, KeyModifiers},
    },
    layout::Rect,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::config::Config;

pub fn configure_new() -> Result<Config> {
    configure_existent(Config::default())
}

pub fn configure_existent(mut config: Config) -> Result<Config> {
    ratatui::run(|term| app(term, &mut config))?;

    Ok(config)
}

fn app(terminal: &mut DefaultTerminal, config: &mut Config) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;

        let event = crossterm::event::read()?;

        if event == Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty())) {
            break;
        }
    }

    Ok(())
}

fn render(frame: &mut Frame) {
    let main_area = Rect::new(0, 0, frame.area().width, frame.area().height);

    let main_menu = Paragraph::new(
        "
Configure SMTP
Configure RAWG
Configure timing",
    )
    .block(
        Block::new()
            .title("Main Menu")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    frame.render_widget(main_menu, main_area);
}
