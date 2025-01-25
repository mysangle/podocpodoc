use crate::prelude::*;
use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

#[derive(Clone, Copy)]
pub enum System {
    Resize(Size),
    Dismiss,
    Search,
    TypeableCommand,
    Action,
    Insert,
    Append,
    OpenBelow,
    OpenAbove,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Enter) {
            Ok(Self::Action)
        } else {
            match code {
                Char(':') => Ok(Self::TypeableCommand),
                Char('i') => Ok(Self::Insert),
                Char('a') => Ok(Self::Append),
                Char('/') => Ok(Self::Search),
                Char('o') => Ok(Self::OpenBelow),
                Char('O') => Ok(Self::OpenAbove),
                _ => Err(format!(
                    "Unsupported key code {code:?} or modifier {modifiers:?}"
                ))
            }
        }
    }
}
