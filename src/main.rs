#![no_std]
#![no_main]

use defmt::{info, error, warn};
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_time::{Delay, Timer};
use ms5611_rs::{Ms5611, Oversampling};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 1. Initialize STM32F405 peripherals with default clock trees
    let p = embassy_stm32::init(Default::default());

    info!("System Booted. Initializing STM32F405RGT7TR Hardware...");

    // 2. Configure I2C1 operating at standard Fast Mode (400 kHz)
    let mut i2c_config = Config::default();
    i2c_config.frequency = Hertz(400_000);

    // Bind hardware blocks to pins PB6 (SCL) and PB7 (SDA)
    let i2c_peripheral = I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        i2c_config,
    );

    // 3. Initialize MS5611 driver via I2C
    // The driver automatically queries the sensor's factory PROM calibration mathematical models
    /// TRUE: High CSB pin (0x76), LOW: CSB low pin (0x77)
    let mut barometer = match Ms5611::new_i2c(i2c_peripheral, true) {
        Ok(device) => {
            info!("MS561101BA03 Barometer found and calibrated successfully!");
            device
        },
        Err(_) => {
            error!("Initialization Failed: Unable to communicate with MS5611 sensor over I2C.");
            return;
        }
    };

    // 4. Telemetry Loop
    loop {
        // Read sample with High Oversampling (OS_2048) for a balance of speed and precision
        match barometer.get_second_order_sample(Oversampling::Osr2048) {
            Ok(sample) => {
                // Conversion: Sensor returns Pa as integer, convert to hPa
                let pressure_hpa = sample.pressure as f32 / 100.0;
                // Sensor returns hundredths of a degree Celsius, convert to °C
                let temperature_c = sample.temperature as f32 / 100.0;

                info!("Telemetry -> Pressure: {} hPa | Temperature: {} °C", pressure_hpa, temperature_c);
            }
            Err(_) => {
                warn!("Bus Error: Failed to read a sample from the MS5611 sensor.");
            }
        }

        // Yield execution to allow system power-saving sleep states for 1 second
        Timer::after_secs(1).await;
    }
}
