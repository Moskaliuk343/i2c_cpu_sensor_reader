use crate::error::SensorError;
use crate::sensor::{TemperatureReading, ThermalSensor};

pub struct MockSensor {
    address: u8,
    initialized: bool,
    counter: u16,
}

impl MockSensor {
    pub fn new(address: u8) -> Self {
        Self {
            address,
            initialized: false,
            counter: 0,
        }
    }
}

impl ThermalSensor for MockSensor {
    fn initialize(&mut self) -> Result<(), SensorError> {
        if !(0x48..=0x4F).contains(&self.address) {
            return Err(SensorError::DeviceNotFound(self.address));
        }

        self.initialized = true;

        println!(
            "Віртуальний I2C-сенсор ініціалізовано: {:#04X}",
            self.address
        );

        Ok(())
    }

    fn read_temperature(&mut self) -> Result<TemperatureReading, SensorError> {
        if !self.initialized {
            return Err(SensorError::NotInitialized);
        }

        // Імітуємо температуру від 25.0 до 29.5 °C
        let temperature = 25.0 + (self.counter % 10) as f32 * 0.5;
        self.counter = self.counter.wrapping_add(1);

        // Кодуємо температуру у форматі TMP102:
        // 12-бітне число, зсунуте вліво на 4 біти.
        let raw = ((temperature / 0.0625) as i16) << 4;

        let msb = (raw >> 8) as u8;
        let lsb = raw as u8;

        // Декодуємо отримані байти
        let raw_value = i16::from_be_bytes([msb, lsb]);
        let decoded = (raw_value >> 4) as f32 * 0.0625;

        if !(-40.0..=125.0).contains(&decoded) {
            return Err(SensorError::InvalidData);
        }

        Ok(TemperatureReading {
            celsius: decoded,
            raw_bytes: [msb, lsb],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sensor_address() {
        let mut sensor = MockSensor::new(0x48);

        assert!(sensor.initialize().is_ok());
    }

    #[test]
    fn test_invalid_sensor_address() {
        let mut sensor = MockSensor::new(0x55);

        assert!(sensor.initialize().is_err());
    }

    #[test]
    fn test_read_without_initialization() {
        let mut sensor = MockSensor::new(0x48);

        assert!(sensor.read_temperature().is_err());
    }

    #[test]
    fn test_temperature_reading() {
        let mut sensor = MockSensor::new(0x48);

        sensor.initialize().unwrap();

        let reading = sensor.read_temperature().unwrap();

        assert!((reading.celsius - 25.0).abs() < 0.001);
        assert_eq!(reading.raw_bytes, [0x19, 0x00]);
    }

    #[test]
    fn test_temperature_progression() {
        let mut sensor = MockSensor::new(0x48);

        sensor.initialize().unwrap();

        let first = sensor.read_temperature().unwrap();
        let second = sensor.read_temperature().unwrap();

        assert!(second.celsius > first.celsius);
    }
}
