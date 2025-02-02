use std::convert::TryFrom;

use crossterm::event::Event;

mod movecommand;
pub use movecommand::Move;
mod system;
pub use system::System;
mod edit;
pub use edit::Edit;

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl Command {
    pub fn handle_normal_default_event(event: Event) -> Result<Self, String> {
        match event {
            Event::Key(key_event) => System::try_from(key_event).map(Command::System)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| Edit::try_from(key_event).map(Command::Edit))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }

    pub fn handle_normal_prompt_event(event: Event) -> Result<Self, String> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event).map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }

    pub fn handle_insert_event(event: Event) -> Result<Self, String> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event).map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
