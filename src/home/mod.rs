#[cfg(test)]
mod tests;

use crate::device::Device;
use crate::room::{Room, RoomError};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error;
use std::fmt::{Debug, Display, Formatter};

pub struct Home {
    pub name: String,
    rooms: HashMap<String, Room>,
}

impl Home {
    pub fn new(name: String) -> Self {
        Home {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Result<&mut Room, HomeError> {
        let e = self.rooms.entry(room.name.clone());
        match e {
            Entry::Occupied(_) => Err(HomeError::AddRoomError(format!(
                "Комната с таким названием есть уже: {}",
                room.name
            ))),
            Entry::Vacant(v) => Ok(v.insert(room)),
        }
    }

    pub fn find_room_by_name(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }

    pub fn find_room_all(&mut self) -> Vec<&String> {
        self.rooms.iter().map(|(_, room)| &room.name).collect()
    }

    pub fn remove_room_by_name(&mut self, name: &str) -> Result<(), HomeError> {
        self.rooms
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| HomeError::DeleteRoomError(format!("Нет такой комнаты: {}", name)))
    }

    pub fn remove_device_by_name(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), HomeError> {
        let room = self
            .find_room_by_name(room_name)
            .ok_or_else(|| HomeError::OtherRoomError(format!("Нет комнаты {}", room_name)))?;
        room.remove_device_by_name(device_name)?;
        Ok(())
    }

    pub fn device_report(&self) -> Result<String, HomeError> {
        Ok("-".to_string())
    }

    pub fn find_device_by_name(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Option<&mut dyn Device> {
        let entry = self.rooms.entry(room_name.to_string());
        match entry {
            Entry::Vacant(_) => None,
            Entry::Occupied(room) => room.into_mut().find_device_by_name(device_name),
        }
    }
}
#[derive(Debug)]
pub enum HomeError {
    AddRoomError(String),
    DeleteRoomError(String),
    OtherRoomError(String),
    ReportError,
}

impl Display for HomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddRoomError(str) => write!(f, "Ошибка добавления устройства {}", str),
            Self::DeleteRoomError(str) => write!(f, "Ошибка удаления устройства {}", str),
            Self::OtherRoomError(str) => write!(f, "{}", str),
            _ => write!(f, "Ошибка построения отчета"),
        }
    }
}

impl error::Error for HomeError {}

impl From<RoomError> for HomeError {
    fn from(_: RoomError) -> Self {
        todo!()
    }
}
