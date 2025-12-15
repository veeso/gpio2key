#[macro_use]
extern crate log;

mod app_log;
mod cli;
mod config;
mod gpio;
mod input_listener;
mod keyboard;

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use self::config::Config;
use self::gpio::RaspberryGpio;
use self::input_listener::{
    InputListener, InputListenerConfig, KeyConfig, PowerSwitch, RepeatConfig,
};
use self::keyboard::EvdevKeyboard;
use crate::gpio::LinuxGpio;

const DEFAULT_REPEAT_DELAY: Duration = Duration::from_millis(500);
const DEFAULT_REPEAT_RATE: Duration = Duration::from_millis(100);

fn main() -> anyhow::Result<()> {
    let args: cli::Args = argh::from_env();
    app_log::init_app_log(args.log_level)?;

    info!("Starting gpio2key with config file: {:?}", args.config);
    // load configuration
    debug!("Loading configuration...");
    let config = config::Config::load_from_file(&args.config)?;
    debug!("Configuration loaded successfully.");
    log_config(&config);

    // setup keyboard
    debug!("Initializing keyboard device...");
    let keys: Vec<_> = config.keys.iter().map(|k| k.keycode.keycode()).collect();
    let keyboard = EvdevKeyboard::try_new(&keys)?;
    info!("Keyboard device initialized.");

    // run application
    if args.raspberry {
        info!("Running on Raspberry Pi board.");
        run_on_raspberry_pi(config, keyboard)?;
    } else {
        info!("Running on generic Linux system.");
        run_on_linux_generic(config, keyboard, &args.device)?;
    }

    Ok(())
}

fn run_on_raspberry_pi(config: Config, keyboard: EvdevKeyboard) -> anyhow::Result<()> {
    // setup gpios
    debug!("Initializing GPIOs...");
    let keys = config
        .keys
        .iter()
        .map(|k| {
            RaspberryGpio::try_new(k.gpio, k.active_low.unwrap_or(config.default_active_low)).map(
                |gpio| KeyConfig {
                    gpio,
                    keycode: k.keycode,
                    debounce: k.debounce().unwrap_or_else(|| config.default_debounce()),
                    repeat: if k.repeat {
                        Some(RepeatConfig {
                            delay: k.repeat_delay().unwrap_or(DEFAULT_REPEAT_DELAY),
                            rate: k.repeat_rate().unwrap_or(DEFAULT_REPEAT_RATE),
                        })
                    } else {
                        None
                    },
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let power_switches = config
        .power_switches
        .iter()
        .map(|ps| RaspberryGpio::try_new(ps.gpio, ps.active_low).map(|gpio| PowerSwitch { gpio }))
        .collect::<Result<Vec<_>, _>>()?;
    info!("GPIOs initialized.");

    // setup ctrlc handler
    let exit = Arc::new(AtomicBool::default());
    {
        let exit = exit.clone();
        ctrlc::set_handler(move || {
            exit.store(true, std::sync::atomic::Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // setup input listener
    let config = InputListenerConfig {
        exit,
        keyboard,
        keys,
        power_switches,
        poll_interval: config.poll_interval(),
    };
    InputListener::new(config).run();

    Ok(())
}

fn run_on_linux_generic(
    config: Config,
    keyboard: EvdevKeyboard,
    device: &Path,
) -> anyhow::Result<()> {
    // setup gpios
    debug!("Initializing GPIOs...");
    let keys = config
        .keys
        .iter()
        .map(|k| {
            LinuxGpio::try_new(
                device,
                k.gpio,
                k.active_low.unwrap_or(config.default_active_low),
            )
            .map(|gpio| KeyConfig {
                gpio,
                keycode: k.keycode,
                debounce: k.debounce().unwrap_or_else(|| config.default_debounce()),
                repeat: if k.repeat {
                    Some(RepeatConfig {
                        delay: k.repeat_delay().unwrap_or(DEFAULT_REPEAT_DELAY),
                        rate: k.repeat_rate().unwrap_or(DEFAULT_REPEAT_RATE),
                    })
                } else {
                    None
                },
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let power_switches = config
        .power_switches
        .iter()
        .map(|ps| {
            LinuxGpio::try_new(device, ps.gpio, ps.active_low).map(|gpio| PowerSwitch { gpio })
        })
        .collect::<Result<Vec<_>, _>>()?;
    info!("GPIOs initialized.");

    // setup ctrlc handler
    let exit = Arc::new(AtomicBool::default());
    {
        let exit = exit.clone();
        ctrlc::set_handler(move || {
            exit.store(true, std::sync::atomic::Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // setup input listener
    let config = InputListenerConfig {
        exit,
        keyboard,
        keys,
        power_switches,
        poll_interval: config.poll_interval(),
    };
    InputListener::new(config).run();

    Ok(())
}

fn log_config(config: &Config) {
    info!("Configuration:");
    info!(
        "  Default debounce: {}",
        config.default_debounce().as_millis()
    );
    info!("  Default active_low: {}", config.default_active_low);
    info!("  Poll interval: {}", config.poll_interval().as_millis());
    info!("  Keys:");
    for key in &config.keys {
        info!("    GPIO: {}", key.gpio);
        info!("    Keycode: {}", key.keycode);
        if let Some(debounce) = key.debounce() {
            info!("    Debounce (ms): {}", debounce.as_millis());
        }
        info!("    Active Low: {:?}", key.active_low);
        info!("    Repeat: {}", key.repeat);
        if let Some(delay) = key.repeat_delay() {
            info!("    Repeat Delay (ms): {}", delay.as_millis());
        }
        if let Some(rate) = key.repeat_rate() {
            info!("    Repeat Rate (ms): {}", rate.as_millis());
        }
    }
    info!("  Power Switches:");
    for ps in &config.power_switches {
        info!("    GPIO {}", ps.gpio);
        info!("    Active Low: {}", ps.active_low);
    }
}
