use std::fmt;
use std::str::FromStr;

/// Wrapper around [`evdev::KeyCode`] to facilitate deserialization
#[derive(Debug, Clone, Copy)]
pub struct Keycode(evdev::KeyCode);

impl Keycode {
    /// Get the underlying [`evdev::KeyCode`]
    pub fn keycode(&self) -> evdev::KeyCode {
        self.0
    }
}

impl fmt::Display for Keycode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl FromStr for Keycode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "A" => Ok(Keycode(evdev::KeyCode::KEY_A)),
            "B" => Ok(Keycode(evdev::KeyCode::KEY_B)),
            "C" => Ok(Keycode(evdev::KeyCode::KEY_C)),
            "D" => Ok(Keycode(evdev::KeyCode::KEY_D)),
            "E" => Ok(Keycode(evdev::KeyCode::KEY_E)),
            "F" => Ok(Keycode(evdev::KeyCode::KEY_F)),
            "G" => Ok(Keycode(evdev::KeyCode::KEY_G)),
            "H" => Ok(Keycode(evdev::KeyCode::KEY_H)),
            "I" => Ok(Keycode(evdev::KeyCode::KEY_I)),
            "J" => Ok(Keycode(evdev::KeyCode::KEY_J)),
            "K" => Ok(Keycode(evdev::KeyCode::KEY_K)),
            "L" => Ok(Keycode(evdev::KeyCode::KEY_L)),
            "M" => Ok(Keycode(evdev::KeyCode::KEY_M)),
            "N" => Ok(Keycode(evdev::KeyCode::KEY_N)),
            "O" => Ok(Keycode(evdev::KeyCode::KEY_O)),
            "P" => Ok(Keycode(evdev::KeyCode::KEY_P)),
            "Q" => Ok(Keycode(evdev::KeyCode::KEY_Q)),
            "R" => Ok(Keycode(evdev::KeyCode::KEY_R)),
            "S" => Ok(Keycode(evdev::KeyCode::KEY_S)),
            "T" => Ok(Keycode(evdev::KeyCode::KEY_T)),
            "U" => Ok(Keycode(evdev::KeyCode::KEY_U)),
            "V" => Ok(Keycode(evdev::KeyCode::KEY_V)),
            "W" => Ok(Keycode(evdev::KeyCode::KEY_W)),
            "X" => Ok(Keycode(evdev::KeyCode::KEY_X)),
            "Y" => Ok(Keycode(evdev::KeyCode::KEY_Y)),
            "Z" => Ok(Keycode(evdev::KeyCode::KEY_Z)),
            "ENTER" => Ok(Keycode(evdev::KeyCode::KEY_ENTER)),
            "SPACE" => Ok(Keycode(evdev::KeyCode::KEY_SPACE)),
            "ESCAPE" => Ok(Keycode(evdev::KeyCode::KEY_ESC)),
            "UP" => Ok(Keycode(evdev::KeyCode::KEY_UP)),
            "DOWN" => Ok(Keycode(evdev::KeyCode::KEY_DOWN)),
            "LEFT" => Ok(Keycode(evdev::KeyCode::KEY_LEFT)),
            "RIGHT" => Ok(Keycode(evdev::KeyCode::KEY_RIGHT)),
            "LCTRL" => Ok(Keycode(evdev::KeyCode::KEY_LEFTCTRL)),
            "RCTRL" => Ok(Keycode(evdev::KeyCode::KEY_RIGHTCTRL)),
            "LSHIFT" => Ok(Keycode(evdev::KeyCode::KEY_LEFTSHIFT)),
            "RSHIFT" => Ok(Keycode(evdev::KeyCode::KEY_RIGHTSHIFT)),
            "LALT" => Ok(Keycode(evdev::KeyCode::KEY_LEFTALT)),
            "RALT" => Ok(Keycode(evdev::KeyCode::KEY_RIGHTALT)),
            "TAB" => Ok(Keycode(evdev::KeyCode::KEY_TAB)),
            "BACKSPACE" => Ok(Keycode(evdev::KeyCode::KEY_BACKSPACE)),
            "CAPSLOCK" => Ok(Keycode(evdev::KeyCode::KEY_CAPSLOCK)),
            "F1" => Ok(Keycode(evdev::KeyCode::KEY_F1)),
            "F2" => Ok(Keycode(evdev::KeyCode::KEY_F2)),
            "F3" => Ok(Keycode(evdev::KeyCode::KEY_F3)),
            "F4" => Ok(Keycode(evdev::KeyCode::KEY_F4)),
            "F5" => Ok(Keycode(evdev::KeyCode::KEY_F5)),
            "F6" => Ok(Keycode(evdev::KeyCode::KEY_F6)),
            "F7" => Ok(Keycode(evdev::KeyCode::KEY_F7)),
            "F8" => Ok(Keycode(evdev::KeyCode::KEY_F8)),
            "F9" => Ok(Keycode(evdev::KeyCode::KEY_F9)),
            "F10" => Ok(Keycode(evdev::KeyCode::KEY_F10)),
            "F11" => Ok(Keycode(evdev::KeyCode::KEY_F11)),
            "F12" => Ok(Keycode(evdev::KeyCode::KEY_F12)),
            "HOME" => Ok(Keycode(evdev::KeyCode::KEY_HOME)),
            "END" => Ok(Keycode(evdev::KeyCode::KEY_END)),
            "PAGEUP" => Ok(Keycode(evdev::KeyCode::KEY_PAGEUP)),
            "PAGEDOWN" => Ok(Keycode(evdev::KeyCode::KEY_PAGEDOWN)),
            "INSERT" => Ok(Keycode(evdev::KeyCode::KEY_INSERT)),
            "DELETE" => Ok(Keycode(evdev::KeyCode::KEY_DELETE)),
            "0" => Ok(Keycode(evdev::KeyCode::KEY_0)),
            "1" => Ok(Keycode(evdev::KeyCode::KEY_1)),
            "2" => Ok(Keycode(evdev::KeyCode::KEY_2)),
            "3" => Ok(Keycode(evdev::KeyCode::KEY_3)),
            "4" => Ok(Keycode(evdev::KeyCode::KEY_4)),
            "5" => Ok(Keycode(evdev::KeyCode::KEY_5)),
            "6" => Ok(Keycode(evdev::KeyCode::KEY_6)),
            "7" => Ok(Keycode(evdev::KeyCode::KEY_7)),
            "8" => Ok(Keycode(evdev::KeyCode::KEY_8)),
            "9" => Ok(Keycode(evdev::KeyCode::KEY_9)),
            "-" => Ok(Keycode(evdev::KeyCode::KEY_MINUS)),
            "=" => Ok(Keycode(evdev::KeyCode::KEY_EQUAL)),
            "[" => Ok(Keycode(evdev::KeyCode::KEY_LEFTBRACE)),
            "]" => Ok(Keycode(evdev::KeyCode::KEY_RIGHTBRACE)),
            "\\" => Ok(Keycode(evdev::KeyCode::KEY_BACKSLASH)),
            ";" => Ok(Keycode(evdev::KeyCode::KEY_SEMICOLON)),
            "'" => Ok(Keycode(evdev::KeyCode::KEY_APOSTROPHE)),
            "`" => Ok(Keycode(evdev::KeyCode::KEY_GRAVE)),
            "," => Ok(Keycode(evdev::KeyCode::KEY_COMMA)),
            "." => Ok(Keycode(evdev::KeyCode::KEY_DOT)),
            "/" => Ok(Keycode(evdev::KeyCode::KEY_SLASH)),
            "NUMPAD_0" => Ok(Keycode(evdev::KeyCode::KEY_KP0)),
            "NUMPAD_1" => Ok(Keycode(evdev::KeyCode::KEY_KP1)),
            "NUMPAD_2" => Ok(Keycode(evdev::KeyCode::KEY_KP2)),
            "NUMPAD_3" => Ok(Keycode(evdev::KeyCode::KEY_KP3)),
            "NUMPAD_4" => Ok(Keycode(evdev::KeyCode::KEY_KP4)),
            "NUMPAD_5" => Ok(Keycode(evdev::KeyCode::KEY_KP5)),
            "NUMPAD_6" => Ok(Keycode(evdev::KeyCode::KEY_KP6)),
            "NUMPAD_7" => Ok(Keycode(evdev::KeyCode::KEY_KP7)),
            "NUMPAD_8" => Ok(Keycode(evdev::KeyCode::KEY_KP8)),
            "NUMPAD_9" => Ok(Keycode(evdev::KeyCode::KEY_KP9)),
            "NUMPAD_ENTER" => Ok(Keycode(evdev::KeyCode::KEY_KPENTER)),
            "NUMPAD_ADD" => Ok(Keycode(evdev::KeyCode::KEY_KPPLUS)),
            "NUMPAD_SUBTRACT" => Ok(Keycode(evdev::KeyCode::KEY_KPMINUS)),
            "NUMPAD_MULTIPLY" => Ok(Keycode(evdev::KeyCode::KEY_KPASTERISK)),
            "NUMPAD_DIVIDE" => Ok(Keycode(evdev::KeyCode::KEY_KPSLASH)),
            _ => Err("Unsupported keycode"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Keycode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Keycode::from_str(&s).map_err(serde::de::Error::custom)
    }
}
