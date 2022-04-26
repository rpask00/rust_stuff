use crate::state::{Draft, State};

pub struct Post {
    state: Option<Box<dyn State>>,
    pub content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::from(""),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}


