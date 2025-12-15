use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use crate::config::Keycode;
use crate::gpio::Gpio;
use crate::keyboard::Keyboard;

/// Configuration for an individual key binding
pub struct KeyConfig<GPIO>
where
    GPIO: Gpio,
{
    pub gpio: GPIO,
    pub keycode: Keycode,
    pub debounce: Duration,
    pub repeat: Option<RepeatConfig>,
}

/// Configuration for key auto-repeat
pub struct RepeatConfig {
    pub delay: Duration,
    pub rate: Duration,
}

/// Configuration for an individual power switch
pub struct PowerSwitch<GPIO>
where
    GPIO: Gpio,
{
    pub gpio: GPIO,
}

/// Configuration for the input listener
pub struct InputListenerConfig<K, GPIO>
where
    K: Keyboard,
    GPIO: Gpio,
{
    pub exit: Arc<AtomicBool>,
    pub keyboard: K,
    pub keys: Vec<KeyConfig<GPIO>>,
    pub power_switches: Vec<PowerSwitch<GPIO>>,
    pub poll_interval: Duration,
}
