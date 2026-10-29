use std::env;

pub struct Config {
    pub count: u32,
    pub interval: u64,
    pub address: u8,
}

impl Config {
    pub fn from_args() -> Result<Option<Self>, String> {
        let args: Vec<String> = env::args().skip(1).collect();

        let mut config = Config {
            count: 10,
            interval: 1,
            address: 0x48,
        };

        let mut i = 0;

        while i < args.len() {
            match args[i].as_str() {
                "--help" | "-h" => {
                    Self::print_help();
                    return Ok(None);
                }

                "--count" => {
                    i += 1;
                    let value = args.get(i).ok_or("Не вказано значення --count")?;

                    config.count = value.parse().map_err(|_| "Некоректне значення --count")?;

                    if config.count == 0 || config.count > 1000 {
                        return Err("--count має бути від 1 до 1000".into());
                    }
                }

                "--interval" => {
                    i += 1;
                    let value = args.get(i).ok_or("Не вказано значення --interval")?;

                    config.interval = value
                        .parse()
                        .map_err(|_| "Некоректне значення --interval")?;

                    if config.interval == 0 || config.interval > 60 {
                        return Err("--interval має бути від 1 до 60".into());
                    }
                }

                "--address" => {
                    i += 1;
                    let value = args.get(i).ok_or("Не вказано значення --address")?;

                    let hex = value.trim_start_matches("0x").trim_start_matches("0X");

                    config.address =
                        u8::from_str_radix(hex, 16).map_err(|_| "Некоректна I2C-адреса")?;
                }

                other => {
                    return Err(format!(
                        "Невідомий параметр: {}. Використайте --help",
                        other
                    ));
                }
            }

            i += 1;
        }

        Ok(Some(config))
    }

    fn print_help() {
        println!("I2C CPU Sensor Reader v0.3.0");
        println!("Використання: cargo run -- [параметри]");
        println!();
        println!("Параметри:");
        println!("  --count N       Кількість вимірювань (1-1000)");
        println!("  --interval N    Інтервал у секундах (1-60)");
        println!("  --address HEX   I2C-адреса (типово 0x48)");
        println!("  --help, -h      Показати довідку");
    }
}
