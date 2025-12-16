# gpio2key

[![license-mit](https://img.shields.io/crates/l/gpio2key.svg)](https://opensource.org/licenses/MIT)
[![repo-stars](https://img.shields.io/github/stars/veeso/gpio2key?style=flat)](https://github.com/veeso/gpio2key/stargazers)
[![downloads](https://img.shields.io/crates/d/gpio2key.svg)](https://crates.io/crates/gpio2key)
[![latest-version](https://img.shields.io/crates/v/gpio2key.svg)](https://crates.io/crates/gpio2key)
[![ko-fi](https://img.shields.io/badge/donate-ko--fi-red)](https://ko-fi.com/veeso)
[![conventional-commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

[![build-test](https://github.com/veeso/gpio2key/actions/workflows/cargo.yml/badge.svg)](https://github.com/veeso/gpio2key/actions/workflows/cargo.yml)

## Introduction

gpio2key is a Raspberry Pi and Linux utility written in Rust that maps GPIO pins to keyboard key events using the evdev interface.
It is particularly useful for embedded systems like Raspberry Pi, where physical buttons connected to GPIO pins can
be used to simulate keyboard input.

In my case I've done this to mod the LEGO Gameboy to add physical buttons using a Raspberry Pi Zero 2W on RetroPie OS.

## Usage

```bash
gpio2key --config config.toml
```

Other options:

- `--config <FILE>`: Path to the configuration file (default: `config.toml`)
- `--device <FILE>`: Path to the GPIO character device representing the GPIO chip (default: `/dev/gpiochip0`)
- `--keyboard-name <NAME>`: Set the name of the virtual keyboard device (default: `gpio2key`)
- `--log-level <LEVEL>`: Set the logging level (options: `error`, `warn`, `info`, `debug`, `trace`; default: `info`)
- `--raspberry`: Enable Raspberry Pi specific GPIO setup. This will also setup gpio as pull-up inputs.
- `--help`: Show help information

> [!NOTE]
>
> On Linux, there is no way from `gpio-cdev` to set pull-up or pull-down resistors on GPIO pins.
> So you need to set them up manually before running the program, or use a hardware solution (e.g. external resistors).
> If you are using a Raspberry Pi, you can use the `--raspberry` flag to automatically set GPIO pins as pull-up inputs using the `rppal` crate.

## Configuration

Default configuration file path is `config.toml`, but can be overridden with the `--config` command line argument.

```toml
default_debounce_ms = 20 # default debounce time in milliseconds
default_active_low = true # default active_low setting for keys; if true, key is active when GPIO is low
poll_interval_ms = 5 # polling interval in milliseconds

[[key]]
gpio = 17
keycode = "A"
active_low = true # `default_active_low` by default
debounce_ms = 20 # `default_debounce_ms` by default
repeat = false # disabled by default

[[key]]
gpio = 22
keycode = "UP"
repeat = true
repeat_delay_ms = 300
repeat_rate_ms = 80

[[powerswitch]]
gpio = 27
active_low = false
```

### Parameters

#### Global Settings

- `default_debounce_ms`: Default debounce time in milliseconds for keys (default: `20`)
- `default_active_low`: Default active_low setting for keys; if true, keys are active when GPIO is low (default: `true`)
- `poll_interval_ms`: Polling interval in milliseconds for checking GPIO states (default: `5`)

#### Keys

Keys are defined in the `[[key]]` array.
A key is mapped to a GPIO pin and generates key events when the pin state changes.

Each key can have the following parameters:

- `gpio`: GPIO pin number (required)
- `keycode`: Key code as defined in `input-event-codes.h` (required)
- `active_low`: If true, the key is active when the GPIO pin is low (default: `default_active_low`)
- `debounce_ms`: Debounce time in milliseconds (default: `default_debounce_ms`)
- `repeat`: If true, the key will auto-repeat when held down (default: `false`)
- `repeat_delay_ms`: Delay before auto-repeat starts in milliseconds (default: `500`)
- `repeat_rate_ms`: Interval between auto-repeats in milliseconds (default: `30`)

#### Power Switches

Power switches are defined in the `[[powerswitch]]` array.
A power switch is mapped to a GPIO pin and triggers a system shutdown when activated.
In order to use this feature, the program must be run with sufficient privileges to execute shutdown commands.

Each power switch can have the following parameter:

- `gpio`: GPIO pin number (required)
- `active_low`: If true, the power switch is active when the GPIO pin is low (default: `true`)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
