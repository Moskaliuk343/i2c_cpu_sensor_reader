mod config;
mod error;
mod mock_sensor;
mod sensor;

use std::thread;
use std::time::Duration;

use config::Config;
use mock_sensor::MockSensor;
use sensor::ThermalSensor;

fn main() {
    println!("==================================");
    println!("       I2C CPU Sensor Reader      ");
    println!("==================================");

    let config = match Config::from_args() {
        Ok(Some(config)) => config,
        Ok(None) => return,
        Err(error) => {
            eprintln!("Помилка налаштувань: {}", error);
            std::process::exit(1);
        }
    };

    println!("Версія: 0.3.0");
    println!("ОС: {}", std::env::consts::OS);
    println!("Режим: Віртуальний I2C-сенсор");
    println!("Адреса: {:#04X}", config.address);
    println!("Кількість вимірювань: {}", config.count);
    println!("Інтервал: {} сек.", config.interval);
    println!("----------------------------------");

    let mut sensor = MockSensor::new(config.address);

    if let Err(error) = sensor.initialize() {
        eprintln!("Помилка ініціалізації: {}", error);
        std::process::exit(1);
    }

    println!("\nПочаток моніторингу...\n");

    for number in 1..=config.count {
        match sensor.read_temperature() {
            Ok(reading) => {
                println!(
                    "[{}] Температура: {:.2} °C | RAW: {:02X} {:02X}",
                    number, reading.celsius, reading.raw_bytes[0], reading.raw_bytes[1]
                );
            }

            Err(error) => {
                eprintln!("Помилка читання: {}", error);
            }
        }

        // Не чекаємо після останнього вимірювання
        if number < config.count {
            thread::sleep(Duration::from_secs(config.interval));
        }
    }

    println!("\nМоніторинг завершено.");
}
