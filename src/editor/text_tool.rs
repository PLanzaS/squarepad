use sdl2::event::Event;
use sdl2::keyboard::{Keycode, TextInputUtil};

// Contains the state of any text being inputted
pub struct TextTool {
    text_input: TextInputUtil, // May need to share this later with code tool
    input_string: String,
}

impl TextTool {
    pub fn new(text_input: TextInputUtil) -> TextTool {
        TextTool {
            text_input,
            input_string: String::new(),
        }
    }

    pub fn start_input(&self) {
        self.text_input.start()
    }
    pub fn stop_input(&self) {
        self.text_input.stop()
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::TextInput { text, .. } => {
                self.input_string.push_str(text);
                println!("{}", self.input_string);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Backspace),
                ..
            } => {
                if !self.input_string.is_empty() {
                    self.input_string.pop();
                    println!("{}", self.input_string);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => self.input_string.push('\n'),
            _ => (),
        }
    }

    pub fn paste(&mut self, text: String) {
        if self.text_input.is_active() {
            self.input_string.push_str(&text);
        }
    }
}
