use atomic_float::AtomicF32;
use std::any::{type_name, Any};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};

static SOCKET_DEVICE_ADDRESS: &str = "127.0.0.1:9555";

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
    address: String,
}

impl SocketDevice {
    pub fn new(name: &str, description: &str) -> Self {
        SocketDevice {
            name: name.to_string(),
            description: description.to_string(),
            address: SOCKET_DEVICE_ADDRESS.to_string(),
        }
    }

    async fn do_request(&mut self, command: &str, buf: &mut [u8]) -> Result<(), DeviceError> {
        let mut stream = TcpStream::connect(self.address.clone())
            .await
            .map_err(|e| DeviceError {
                message: e.to_string(),
            })?;
        stream
            .write((command.to_owned() + "\n").as_bytes())
            .await
            .map_err(|e| DeviceError {
                message: e.to_string(),
            })?;
        stream.read(buf).await.map_err(|e| DeviceError {
            message: e.to_string(),
        })?;
        Ok(())
    }

    pub async fn switch(&mut self) -> Result<bool, DeviceError> {
        let mut buf = [0; 1];
        self.do_request("switch", &mut buf).await?;
        Ok(buf[0] == 1)
    }

    pub async fn get_value(&self) -> Result<f32, DeviceError> {
        let mut stream = TcpStream::connect(self.address.clone())
            .await
            .map_err(|e| DeviceError {
                message: e.to_string(),
            })?;
        stream
            .write("getValue\n".as_bytes())
            .await
            .map_err(|e| DeviceError {
                message: e.to_string(),
            })?;
        let mut buf = [0; 4];
        stream.read(&mut buf).await.map_err(|e| DeviceError {
            message: e.to_string(),
        })?;
        Ok(f32::from_be_bytes(buf))
    }

    pub async fn is_on(&mut self) -> Result<bool, DeviceError> {
        let mut buf = [0; 1];
        self.do_request("getState", &mut buf).await?;
        Ok(buf[0] == 1)
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
    device_addr: String,
    value: Arc<AtomicF32>,
}

impl ThermometerDevice {
    pub fn new(name: &str, description: &str, addr: &str) -> Self {
        let result = ThermometerDevice {
            name: name.to_string(),
            description: description.to_string(),
            value: Arc::new(AtomicF32::new(0.0)),
            device_addr: addr.to_string(),
        };
        if !result.device_addr.is_empty() {
            println!("addr: {}", result.device_addr);
            result.listen();
        }
        result
    }

    pub fn get_value(&self) -> f32 {
        self.value.load(Ordering::Relaxed)
    }

    fn listen(&self) {
        let clone_value = self.value.clone();
        let addr_clone = self.device_addr.clone();
        tokio::spawn(async move {
            let socket = UdpSocket::bind(addr_clone.deref()).await.unwrap();
            loop {
                let mut buf = [0; 4];
                socket.recv_from(&mut buf).await.unwrap();
                clone_value.store(f32::from_be_bytes(buf), Ordering::Relaxed);
            }
        });
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
