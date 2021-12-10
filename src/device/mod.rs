use rand::Rng;
use std::any::{type_name, Any};
use std::fmt::{Debug, Display, Formatter};

pub struct DeviceInfo {
    pub name: String,
    pub description: String,
    pub device_type: String,
}

pub trait AsAny {
    fn to_any(&mut self) -> &mut dyn Any;
}

pub trait Device: AsAny {
    fn get_info(&self) -> DeviceInfo;
    fn get_type(&self) -> String {
        type_name::<Self>().to_string()
    }
}

pub struct SocketDevice {
    name: String,
    description: String,
    is_on: bool,
    value: f32,
}

impl SocketDevice {
    pub fn new(name: &str, description: &str) -> Self {
        SocketDevice {
            name: name.to_string(),
            description: description.to_string(),
            value: 0.0,
            is_on: false,
        }
    }

    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

impl AsAny for SocketDevice {
    fn to_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Device for SocketDevice {
    fn get_info(&self) -> DeviceInfo {
        DeviceInfo {
            device_type: self.get_type(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}

pub struct ThermometerDevice {
    name: String,
    description: String,
}

impl ThermometerDevice {
    pub fn new(name: &str, description: &str) -> Self {
        ThermometerDevice {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
    pub fn get_value(&self) -> Result<i32, DeviceError> {
        Some(rand::thread_rng().gen::<i32>())
            .filter(|&x| x % 2 == 0)
            .ok_or(DeviceError {
                message: "bzzzz".to_string(),
            })
    }
}

impl AsAny for ThermometerDevice {
    fn to_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Device for ThermometerDevice {
    fn get_info(&self) -> DeviceInfo {
        DeviceInfo {
            device_type: self.get_type(),
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}
#[derive(Debug)]
pub struct DeviceError {
    message: String,
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ошибка устройства {}", self.message)
    }
}

impl std::error::Error for DeviceError {}
