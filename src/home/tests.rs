use super::*;

#[test]
fn test_new_home() {
    let name = "Это дом".to_string();
    let home = Home::new(name.clone());
    assert_eq!(home.name, name)
}

#[test]
fn test_add_unique_room() {
    let mut home = Home::new("Это дом".to_string());
    let result = home.add_room(Room::new("комната", 3));
    assert!(result.is_ok());
    let result = home.add_room(Room::new("комната", 3));
    assert!(result.is_err());
}

#[test]
fn test_delete_room() {
    let mut home = Home::new("Это дом".to_string());
    let _ = home.add_room(Room::new("комната", 3));
    let _ = home.add_room(Room::new("комнат1", 3));

    let result = home.remove_room_by_name("комната");
    assert!(result.is_ok());

    assert_eq!(home.rooms.len(), 1);

    let result = home.remove_room_by_name("комната");
    assert!(result.is_err());
}
