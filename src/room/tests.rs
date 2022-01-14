use super::*;
use crate::device::SocketDevice;
#[test]
fn test_new_room() {
    let _ = Room::new("комната", 5);
}

#[test]
fn test_add_device() {
    let mut room = Room::new("комната", 5);
    let _ = room
        .add_device(Box::new(SocketDevice::new("розетка", "с описанием")))
        .unwrap();
}

#[test]
fn test_unique_device_name() {
    let mut room = Room::new("комната", 5);
    let _ = room.add_device(Box::new(SocketDevice::new("розетка", "с описанием")));
    let result = room.add_device(Box::new(SocketDevice::new("розетка", "с описанием")));
    assert!(result.is_err());
}

#[test]
fn test_find_device() {
    let mut room = Room::new("комната", 5);
    let _ = room.add_device(Box::new(SocketDevice::new("розетка", "с описанием")));

    let device_name = "розетка";
    let fnd_device = room.find_device_by_name(device_name);

    assert!(fnd_device.is_some());
    assert_eq!(fnd_device.unwrap().get_info().name, device_name);
}

#[test]
fn test_delete_device() {
    let mut room = Room::new("комната", 5);
    let _ = room.add_device(Box::new(SocketDevice::new("розетка", "с описанием")));
    let _ = room.add_device(Box::new(SocketDevice::new("еще розетка", "с описанием")));
    let _ = room.remove_device_by_name("розетка").unwrap();

    assert_eq!(room.slots.iter().filter(|s| s.is_some()).count(), 1);
    room.find_device_by_name("еще розетка").unwrap();
}
