# Astro Avionics - STM32F405 Barometer Firmware

An embedded asynchronous Rust firmware designed to interface an STM32F405 microcontroller with an MS5611 barometric pressure and temperature sensor to maximize telemetry throughput, measurement accuracy, and energy efficiency.

---

## Hardware & Device Specifications

### Microcontroller
* **MCU:** STM32F405RGT6 / STM32F405RGT7TR (ARM Cortex-M4F with FPU)
* **Architecture:** `thumbv7em-none-eabihf`
* **Clock & Timers:** Hardware clock tree with hardware timer integration (`TIM2` time driver)

### Sensor
* **Device:** MS5611-01BA03 High-Resolution Barometric Pressure and Temperature Sensor
* **Interface:** I2C1 (Fast Mode @ 400 kHz)
* **Sampling:** 2nd-order compensated sampling with `OSR_2048` oversampling for an optimal trade-off between conversion speed, signal noise reduction, and power consumption.

### Pinout & Wiring

| STM32F405 Pin | MS5611 Sensor Pin | Function / Description |
| :--- | :--- | :--- |
| **PB6** | SCL | I2C1 Clock (400 kHz) |
| **PB7** | SDA | I2C1 Data |
| **3.3V** | VDD / VCC | Power Supply (3.3V DC) |
| **GND** | GND | Common Ground |
| **SWDIO / SWCLK / GND** | ST-Link / J-Link | SWD Debug & Flashing Interface |

---

## Software & Firmware Stack

* **Language & Runtime:** `#![no_std]` Rust (2021 edition)
* **Async Framework:** [Embassy](https://embassy.dev/) (`embassy-stm32`, `embassy-executor`, `embassy-time`)
* **Sensor Driver:** [`ms5611-rs`](https://crates.io/crates/ms5611-rs)
* **Logging & Telemetry:** `defmt` over Real-Time Transfer (RTT) via `defmt-rtt` and `panic-probe`
* **Target Runner / Debugging:** `probe-rs`

---

## Basic Setup & Getting Started

### 1. Prerequisites

Ensure the Rust toolchain and the ARM Cortex-M4F compilation target are installed:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add the Cortex-M4F compilation target
rustup target add thumbv7em-none-eabihf

# Install probe-rs for flashing and RTT logging
cargo install probe-rs-tools --locked
```

### 2. Connect Hardware

1. Connect the MS5611 sensor to the STM32F405 board according to the pinout table above. Ensure pull-up resistors are present on the I2C lines if not integrated into the sensor breakout module.
2. Connect an SWD programmer/debugger (such as an ST-Link V2/V3 or J-Link) to the STM32F405 SWD header (SWDIO, SWCLK, GND, 3.3V).
3. Plug the debugger into your development machine via USB.

### 3. Build and Flash

The repository is pre-configured in `.cargo/config.toml` to target `thumbv7em-none-eabihf` and use `probe-rs` as the runner.

Run the application directly:

```bash
# Build, flash, and stream RTT telemetry logs in release mode
cargo run --release
```

To compile without flashing:

```bash
cargo build --release
```

---

## Telemetry Output

Upon boot, the firmware initializes I2C1, reads the factory PROM calibration coefficients from the MS5611, and emits calibrated barometric pressure (hPa) and temperature (°C) telemetry once per second over RTT:

```text
INFO  System Booted. Initializing STM32F405RGT7TR Hardware...
INFO  MS561101BA03 Barometer found and calibrated successfully!
INFO  Telemetry -> Pressure: 1013.25 hPa | Temperature: 22.45 °C
INFO  Telemetry -> Pressure: 1013.23 hPa | Temperature: 22.46 °C
```
