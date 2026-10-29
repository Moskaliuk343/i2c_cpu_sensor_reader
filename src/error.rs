use std::fmt;

#[derive(Debug)]
pub enum SensorError {
    DeviceNotFound(u8),
    NotInitialized,
    InvalidData,
    BusError(String),
}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorError::DeviceNotFound(addr) => {
                write!(f, "Сенсор за адресою {:#04X} не знайдено", addr)
            }
            SensorError::NotInitialized => {
                write!(f, "Сенсор не ініціалізовано")
            }
            SensorError::InvalidData => {
                write!(f, "Отримано некоректні дані температури")
            }
            SensorError::BusError(message) => {
                write!(f, "Помилка I2C: {}", message)
            }
        }
    }
}

impl std::error::Error for SensorError {}
