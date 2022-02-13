pub mod device;
pub mod home;
pub mod room;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::device::SocketDevice;
    use crate::device::ThermometerDevice;
    use crate::room::Room;
    use tokio::time::{sleep, Duration};

    // Дом имеет название и содержит несколько помещений.
    #[test]
    fn test_home_name() {
        let mut home = home::Home::new("Дом".to_string());
        home.add_room(Room::new("комната1", 2)).unwrap();
        home.add_room(Room::new("комната2", 2)).unwrap();
        home.add_room(Room::new("комната3", 2)).unwrap();
    }
    // Библтотека позволяет запросить список помещений, добавлять и удалять помещения в доме.
    #[test]
    fn test_home_rooms() {
        let mut home = home::Home::new("Дом".to_string());
        home.add_room(Room::new("комната1", 2)).unwrap();
        home.add_room(Room::new("комната2", 2)).unwrap();
        home.add_room(Room::new("комната3", 2)).unwrap();
        {
            let rooms = home.find_room_all();
            assert_eq!(rooms.len(), 3);
        }
        home.remove_room_by_name("комната2").unwrap();
        let rooms = home.find_room_all();
        assert_eq!(rooms.len(), 2);
    }
    // Помещение имеет уникальное название и содержит несколько устройств.
    #[test]
    fn test_room_unique_name() {
        let mut home = home::Home::new("Дом".to_string());
        let room = home.add_room(Room::new("комната1", 3)).unwrap();
        room.add_device(Box::new(SocketDevice::new("розетка", "описание")))
            .unwrap();
        room.add_device(Box::new(SocketDevice::new("розетка1", "описание")))
            .unwrap();
        room.add_device(Box::new(ThermometerDevice::new(
            "температурометр",
            "описание",
            "",
        )))
        .unwrap();
        let result = home.add_room(Room::new("комната1", 2));
        assert!(result.is_err());
    }

    // Устройство имеет уникальное в рамках помещения название, тип и описание.
    #[test]
    fn test_device_unique_device_name() {
        let mut home = home::Home::new("Дом".to_string());
        let room = home.add_room(Room::new("комната1", 3)).unwrap();
        room.add_device(Box::new(SocketDevice::new("розетка", "описание")))
            .unwrap();
        let result = room.add_device(Box::new(SocketDevice::new("розетка", "описание")));
        assert!(result.is_err());
    }

    // Библтотека позволяет добавлять, получать и удалять любое устройство в доме. Получать список устройств в помещении.
    #[test]
    fn test_device_add_delete_list() {
        let mut home = home::Home::new("Дом".to_string());
        let room = home.add_room(Room::new("комната1", 3)).unwrap();
        room.add_device(Box::new(SocketDevice::new("розетка", "описание")))
            .unwrap();
        room.add_device(Box::new(SocketDevice::new("еще розетка", "описание")))
            .unwrap();
        let fnd_devices = room.find_device_all();
        assert_eq!(fnd_devices.len(), 2);
        room.remove_device_by_name("розетка").unwrap();
        home.find_device_by_name("комната1", "еще розетка").unwrap();
    }

    // Умная розетка позволяет включать и выключать себя. Предоставляет информацию о текущем состоянии и потребляемой мощности.
    //  Если сервер не работает тест не проходит
    #[tokio::test]
    async fn test_socket_device() {
        let mut home = home::Home::new("Дом".to_string());
        let room = home.add_room(Room::new("комната1", 2)).unwrap();
        let socket = room
            .add_device(Box::new(SocketDevice::new("розетка", "и ее описание")))
            .unwrap()
            .to_any()
            .downcast_mut::<SocketDevice>()
            .unwrap();
        let state = socket.is_on().await.unwrap();
        assert_eq!(socket.is_on().await.unwrap(), state);
        socket.switch().await.unwrap();
        assert_eq!(socket.is_on().await.unwrap(), !state);
        assert_eq!(socket.get_value().await.unwrap(), 123.);
    }
    //Термометр позволяет узнать температуру.
    #[tokio::test]
    async fn test_thermometer_device() {
        let mut home = home::Home::new("Дом".to_string());
        let room = home.add_room(Room::new("комната1", 2)).unwrap();
        let thermometer = room
            .add_device(Box::new(ThermometerDevice::new(
                "термометр",
                "с описанием",
                "127.0.0.1:9555",
            )))
            .unwrap()
            .to_any()
            .downcast_mut::<ThermometerDevice>()
            .unwrap();
        sleep(Duration::from_secs(10)).await;
        let val = thermometer.get_value();
        assert_ne!(val, 0.0);
        sleep(Duration::from_secs(10)).await;
        assert_ne!(val, thermometer.get_value());
    }

    // Библиотека позволяет строить отчёт о состоянии всех устройств в доме.
    #[test]
    fn test_report() {
        let mut home = home::Home::new("Дом".to_string());
        home.add_room(Room::new("комната1", 2)).unwrap();
        home.device_report().unwrap();
    }
}
