use std::time::Duration;

use crossterm::event;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::Paragraph,
};
use rattles::presets::prelude as presets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Fill(1),
                        Constraint::Length(1),
                        Constraint::Fill(1),
                    ])
                    .split(area);

                let spinner =
                    Paragraph::new(presets::dots().current_frame()).alignment(Alignment::Center);

                frame.render_widget(spinner, layout[1]);
            })?;

            if event::poll(Duration::from_millis(10))? {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(33));
        }

        Ok(())
    })
}
