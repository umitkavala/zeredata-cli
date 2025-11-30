use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct SearchBox {
    pub query: String,
    pub cursor_position: usize,
    pub active: bool,
}

impl SearchBox {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            cursor_position: 0,
            active: false,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.cursor_position = 0;
    }

    pub fn handle_char(&mut self, c: char) {
        if self.active {
            self.query.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.active && self.cursor_position > 0 {
            self.query.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let border_style = if self.active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };

        let text = vec![
            Line::from(vec![
                Span::styled("🔍 ", Style::default().fg(Color::Yellow)),
                Span::styled(&self.query, Style::default().fg(Color::White)),
                if self.active {
                    Span::styled("_", Style::default().fg(Color::Yellow))
                } else {
                    Span::raw("")
                },
            ])
        ];

        let search = Paragraph::new(text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Search (press / to activate, Esc to clear)")
                .border_style(border_style));

        f.render_widget(search, area);
    }

    /// Fuzzy match using simple substring matching
    pub fn matches(&self, text: &str) -> bool {
        if self.query.is_empty() {
            return true;
        }

        let query_lower = self.query.to_lowercase();
        let text_lower = text.to_lowercase();

        // Simple fuzzy matching: check if all chars appear in order
        let mut text_chars = text_lower.chars();
        for query_char in query_lower.chars() {
            match text_chars.find(|&c| c == query_char) {
                Some(_) => continue,
                None => return false,
            }
        }

        true
    }

    /// Calculate match score (higher is better)
    pub fn match_score(&self, text: &str) -> i32 {
        if self.query.is_empty() {
            return 0;
        }

        let text_lower = text.to_lowercase();
        let query_lower = self.query.to_lowercase();

        let mut score = 0;

        // Exact match
        if text_lower == query_lower {
            score += 1000;
        }
        // Starts with query
        else if text_lower.starts_with(&query_lower) {
            score += 500;
        }
        // Contains query as substring
        else if text_lower.contains(&query_lower) {
            score += 100;
        }

        // Fuzzy match bonus
        if self.matches(text) {
            score += 10;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        let mut search = SearchBox::new();
        search.query = "wrhs".to_string();

        assert!(search.matches("warehouse"));
        assert!(search.matches("Warehouse Test Job"));
        assert!(!search.matches("robot arm"));
    }

    #[test]
    fn test_match_score() {
        let mut search = SearchBox::new();
        search.query = "test".to_string();

        assert!(search.match_score("test") > search.match_score("testing"));
        assert!(search.match_score("test job") > search.match_score("my test"));
    }
}
