use crate::error::SensorError;

#[derive(Debug, Clone, Copy)]
pub struct TemperatureReading {
    pub celsius: f32,
    pub raw_bytes: [u8; 2],
}

pub trait ThermalSensor {
    fn initialize(&mut self) -> Result<(), SensorError>;

    fn read_temperature(&mut self) -> Result<TemperatureReading, SensorError>;
}
