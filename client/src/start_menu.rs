use ratatui::{layout::Rect, widgets::{Block, BorderType, Borders, List, Paragraph, Tabs}};
use ratatui::prelude::*;

use crate::menu;

pub struct StartMenu {

}
impl menu::Menu for StartMenu {
    fn menu_type(&self) -> menu::MenuState {
        menu::MenuState::Start
    }

    fn update(&mut self, input: String) {
        
    }

    fn ui(&self, frame: &mut Frame) {
        frame.render_widget(
            List::new(["lorem", "ipsum", "dolor"])
                .block(Block::default().title("servers").title_style(Modifier::BOLD).border_type(BorderType::Rounded).borders(Borders::ALL)),
            Rect { x: 0, y: 0, width: 20, height: frame.size().height }
        )
    }
}
