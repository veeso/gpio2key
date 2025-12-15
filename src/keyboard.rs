mod evdev_keyboard;
#[cfg(test)]
mod mock;

pub use self::evdev_keyboard::EvdevKeyboard;
#[cfg(test)]
#[allow(unused)]
pub use self::mock::MockKeyboard;

/// Keyboard trait definition
pub trait Keyboard {
    /// Emit a key up event
    fn key_up(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()>;

    /// Emit a key down event
    fn key_down(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()>;

    /// Emit a key repeat event
    fn key_repeat(&mut self, keycode: evdev::KeyCode) -> anyhow::Result<()>;
}
