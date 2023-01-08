use std::mem;

use space10::Game;

use crate::parser::command::{Command, Error};

use super::{Controller, SubView};

#[derive(Clone, PartialEq)]
pub enum Payload {
    Command(Command),
    Error(Error),
}

#[derive(Clone, PartialEq)]
pub struct Message {
    pub id: usize,
    pub buffer: String,
    pub payload: Payload,
}

#[derive(Clone)]
pub struct ConsoleContext {
    pub messages: Vec<Message>,
    pub buffer: String,
    generation: u8,
    message_id_counter: usize,
}

#[derive(Clone, Debug)]
pub enum ConsoleAction {
    UpdateBuffer(String),
    FlushBuffer,
}

impl Default for ConsoleContext {
    fn default() -> Self {
        Self {
            messages: Vec::with_capacity(10),
            buffer: Default::default(),
            generation: 0,
            message_id_counter: 0,
        }
    }
}

impl PartialEq for ConsoleContext {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
    }
}

impl ConsoleContext {
    fn flush(&mut self, game: &Game) {
        let buffer = mem::replace(&mut self.buffer, "".into());
        let trimmed = buffer.trim();

        if !trimmed.is_empty() {
            let result = Command::parse(&game, &trimmed);

            let payload = match result {
                Ok(command) => Payload::Command(command),
                Err(error) => Payload::Error(error),
            };

            let id = self.message_id_counter;
            self.message_id_counter += 1;

            self.messages.push(Message {
                id,
                buffer: trimmed.to_owned(),
                payload,
            });
        }
    }
}

impl SubView for ConsoleContext {
    type Action = ConsoleAction;

    fn inplace_update(controller: &mut Controller, action: Self::Action) -> bool {
        let game = &controller.game;
        match action {
            ConsoleAction::UpdateBuffer(buffer) => {
                controller.console.buffer = buffer;
            }
            ConsoleAction::FlushBuffer => controller.console.flush(&game),
        };

        true
    }
}
