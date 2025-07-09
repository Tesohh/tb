use color_eyre::eyre::Result;
use ratatui::{
    crossterm::event,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{self, Block, Clear, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App { counter: 0 };
    let terminal = ratatui::init();
    let result = app.run(terminal);
    ratatui::restore();
    result
}

struct App {
    counter: u64,
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Char('q') => break Ok(()),
                    event::KeyCode::Char(' ') => self.counter += 1,
                    event::KeyCode::Backspace => self.counter -= 1,
                    event::KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = center(
            frame.area(),
            Constraint::Percentage(20),
            Constraint::Length(3), // top and bottom border + content
        );
        let popup =
            Paragraph::new(format!("{}", self.counter)).block(Block::bordered().title("Popup"));
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }
}

fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
