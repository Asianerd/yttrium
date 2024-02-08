use ratatui::Frame;

pub trait Menu {
    // fn new() -> Box<dyn Menu>;
    fn menu_type(&self) -> MenuState;
    fn update(&mut self, input: String); // per-input update
    fn ui(&self, frame: &mut Frame);
}

#[derive(PartialEq, Eq, Hash)]
pub enum MenuState {
    Start,
    Login,
    SignUp,
    Chat
}