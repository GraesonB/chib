pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn handle_event(&mut self, event: crossterm::event::Event) {}

    pub fn update(&mut self) {}

    pub fn render(&self, frame: &mut Frame) {
        let hello = Paragraph::new("Hello from TUIIIIIIIIIII");
        frame.render_widget(hello, frame.area());
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}
