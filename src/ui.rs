use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("temp")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let viewport_height = area.height as usize;

        let upper_bound = viewport_height.saturating_sub(4).min(self.cursor_line);

        let lower_bound = (upper_bound + viewport_height).min(self.text.len());

        let text = format!(
            "Ebook reader, use j to move down, k to move up\n
             ----------------------------------------------\n{}",
            self.text[upper_bound..lower_bound]
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    if i + upper_bound == self.cursor_line {
                        format!("> {}", line)
                    } else {
                        format!("  {}", line)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .centered();

        paragraph.render(area, buf);
    }
}
