use crate::device::Device;
use std::fmt::Debug;
use std::ops::DerefMut;
use thiserror::Error;

#[cfg(test)]
mod tests;

pub struct Room {
    pub name: String,
    pub slots: Vec<Option<Box<dyn Device>>>,
}

impl Room {
    pub fn new(name: &str, volume: u32) -> Self {
        let mut result = Room {
            name: name.to_string(),
            slots: vec![],
        };
        for _ in 0..volume {
            result.slots.push(None);
        }
        result
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) -> Result<&mut dyn Device, RoomError> {
        let new_device_name = (*device).get_info().name;
        if self.slots.iter().any(|slot| {
            slot.is_some() && (*slot.as_ref().unwrap()).get_info().name == new_device_name
        }) {
            return Err(RoomError::AddDeviceError(format!(
                "Такое устройство есть уже: {}",
                &new_device_name
            )));
        }

        let empty_slot: &mut Option<Box<dyn Device>> = self
            .slots
            .iter_mut()
            .find(|slot| slot.is_none())
            .ok_or_else(|| RoomError::AddDeviceError("Нет мест :(".to_string()))?;
        Ok(empty_slot.insert(device).deref_mut())
    }

    pub fn find_device_by_name(&mut self, name: &str) -> Option<&mut dyn Device> {
        let a = self
            .slots
            .iter_mut()
            .find(|slot| slot.is_some() && slot.as_ref().unwrap().get_info().name == *name);
        match a {
            None => None,
            Some(a) => Some(a.as_mut().unwrap().deref_mut()),
        }
    }

    pub fn find_device_all(&self) -> Vec<String> {
        self.slots
            .iter()
            .filter(|slot| slot.is_some())
            .map(|slot| (*slot.as_ref().unwrap()).get_info().name)
            .collect()
    }

    pub fn remove_device_by_name(&mut self, name: &str) -> Result<(), RoomError> {
        let slot_idx = self
            .slots
            .iter()
            .position(|slot| slot.is_some() && (*slot.as_ref().unwrap()).get_info().name == name)
            //                          А это как то можно проще? ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            .ok_or_else(|| {
                RoomError::RemoveDeviceError(format!("Нет такого устройства: {}", name))
            })?;
        self.slots[slot_idx] = None;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RoomError {
    #[error("Ошибка удаления комнаты {0}")]
    RemoveDeviceError(String),
    #[error("Ошибка добавления комнаты {0}")]
    AddDeviceError(String),
}
