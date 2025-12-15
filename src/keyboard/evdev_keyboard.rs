use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, KeyCode, KeyEvent};

use crate::keyboard::Keyboard;

const KEY_DOWN: i32 = 1;
const KEY_UP: i32 = 0;
const KEY_REPEAT: i32 = 2;

/// A evdev virtual keyboard implementation
pub struct EvdevKeyboard {
    device: VirtualDevice,
}

impl EvdevKeyboard {
    /// Create a new [`EvdevKeyboard`] that listens to the specified keycodes
    pub fn try_new(listen_to: &[KeyCode]) -> anyhow::Result<Self> {
        let mut keys = AttributeSet::new();
        for &key in listen_to {
            keys.insert(key);
        }

        VirtualDevice::builder()?
            .name("gpio2key virtual keyboard")
            .with_keys(&keys)?
            .build()
            .map(|device| EvdevKeyboard { device })
            .map_err(|e| anyhow::anyhow!("Failed to create evdev virtual keyboard: {}", e))
    }

    /// Emit a key event with the specified value
    fn emit(&mut self, keycode: evdev::KeyCode, value: i32) -> anyhow::Result<()> {
        let ev = *KeyEvent::new_now(keycode, value);

        self.device
            .emit(&[ev])
            .map_err(|e| anyhow::anyhow!("Failed to emit key event: {}", e))
    }
}

impl Keyboard for EvdevKeyboard {
    fn key_down(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()> {
        self.emit(keycode, KEY_DOWN)
    }

    fn key_repeat(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()> {
        self.emit(keycode, KEY_REPEAT)
    }

    fn key_up(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()> {
        self.emit(keycode, KEY_UP)
    }
}
