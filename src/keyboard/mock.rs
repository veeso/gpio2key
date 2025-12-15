use crate::keyboard::Keyboard;

const KEY_DOWN: i32 = 1;
const KEY_UP: i32 = 0;
const KEY_REPEAT: i32 = 2;

#[derive(Debug, Default)]
pub struct MockKeyboard {
    pub events: Vec<(evdev::KeyCode, i32)>,
}

impl Keyboard for MockKeyboard {
    fn key_down(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()> {
        self.events.push((keycode, KEY_DOWN));
        Ok(())
    }

    fn key_repeat(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()> {
        self.events.push((keycode, KEY_REPEAT));
        Ok(())
    }

    fn key_up(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()> {
        self.events.push((keycode, KEY_UP));
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_mock_keyboard() {
        let mut keyboard = MockKeyboard::default();
        keyboard.key_down(evdev::KeyCode::KEY_A).unwrap();
        keyboard.key_repeat(evdev::KeyCode::KEY_A).unwrap();
        keyboard.key_up(evdev::KeyCode::KEY_A).unwrap();
        assert_eq!(
            keyboard.events,
            vec![
                (evdev::KeyCode::KEY_A, KEY_DOWN),
                (evdev::KeyCode::KEY_A, KEY_REPEAT),
                (evdev::KeyCode::KEY_A, KEY_UP),
            ]
        );
    }
}
