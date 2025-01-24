use crossterm::event::{
    KeyCode::{Char, Down, End, Home, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyModifiers,
};

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    HalfPageUp,
    HalfPageDown,
    PageUpOneLine,
    PageDownOneLine,
    StartOfLine,
    EndOfLine,
    Up,
    Left,
    Right,
    Down,
}
impl TryFrom<KeyEvent> for Move {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                PageDown => Ok(Self::PageDown),
                PageUp => Ok(Self::PageUp),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('b') => Ok(Self::PageUp),
                Char('f') => Ok(Self::PageDown),
                Char('u') => Ok(Self::HalfPageUp),
                Char('d') => Ok(Self::HalfPageDown),
                Char('y') => Ok(Self::PageUpOneLine),
                Char('e') => Ok(Self::PageDownOneLine),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
