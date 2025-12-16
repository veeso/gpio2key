mod config;
mod state;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

pub use self::config::{InputListenerConfig, KeyConfig, PowerSwitch, RepeatConfig};
use self::state::KeyState;
use crate::gpio::{Gpio, GpioValue};
use crate::input_listener::state::OutEvent;
use crate::keyboard::Keyboard;

/// Input listener.
///
/// The input listener monitors GPIOs and emits key events via the keyboard interface.
pub struct InputListener<K, GPIO>
where
    K: Keyboard,
    GPIO: Gpio,
{
    exit: Arc<AtomicBool>,
    keyboard: K,
    keys: Vec<KeyState<GPIO>>,
    power_switches: Vec<PowerSwitch<GPIO>>,
    poll_interval: Duration,
}

impl<K, G> InputListener<K, G>
where
    K: Keyboard,
    G: Gpio,
{
    /// Create a new input listener with the given configuration
    pub fn new(config: InputListenerConfig<K, G>) -> Self {
        InputListener {
            exit: config.exit,
            keyboard: config.keyboard,
            keys: config.keys.into_iter().map(KeyState::from).collect(),
            power_switches: config.power_switches,
            poll_interval: config.poll_interval,
        }
    }

    /// Run the input listener
    pub fn run(mut self) {
        while !self.exit.load(std::sync::atomic::Ordering::SeqCst) {
            for key in &mut self.keys {
                Self::handle_key_poll(key, &mut self.keyboard);
            }
            for switch in &mut self.power_switches {
                Self::handle_power_switch_poll(switch, &self.exit);
            }
            trace!("tick");
            std::thread::sleep(self.poll_interval);
        }
    }

    /// Handle polling of a single key
    fn handle_key_poll(key: &mut KeyState<G>, keyboard: &mut K) {
        // read value
        trace!("Polling key {:?}", key.keycode);
        let Ok(value) = key.gpio.read() else {
            error!("Failed to read GPIO for key {:?}", key.keycode);
            return;
        };
        trace!("Read GPIO value {:?} for key {:?}", value, key.keycode);
        // handle value
        let res = match key.handle_gpio_value(value) {
            OutEvent::None => Ok(()),
            OutEvent::Press => {
                info!("Key {:?} pressed", key.keycode);
                keyboard.key_down(key.keycode.keycode())
            }
            OutEvent::Release => {
                info!("Key {:?} released", key.keycode);
                keyboard.key_up(key.keycode.keycode())
            }
            OutEvent::Repeat => {
                info!("Key {:?} repeat", key.keycode);
                keyboard.key_repeat(key.keycode.keycode())
            }
        };
        if let Err(e) = res {
            error!("Failed to send key event for key {:?}: {}", key.keycode, e);
        }
    }

    /// Handle polling of a single power switch
    fn handle_power_switch_poll(switch: &mut PowerSwitch<G>, exit: &Arc<AtomicBool>) {
        let value = match switch.gpio.read() {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to read GPIO for power switch: {}", e);
                return;
            }
        };
        if value == GpioValue::Enabled {
            warn!("Power switch activated, shutting down system");
            #[cfg(target_os = "linux")]
            unsafe {
                if libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF) != 0 {
                    error!(
                        "Failed to shut down system: {}",
                        std::io::Error::last_os_error()
                    );
                }
            }
            // set exit
            exit.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
