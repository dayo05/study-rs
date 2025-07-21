#[derive(Debug)]
pub enum Event {
    Arrived(i32),
    DoorUpdate(bool),
    PressFloorButton(i32),
    PressDirectionButton(i32, Direction),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

pub fn car_arrived(floor: i32) -> Event {
    Event::Arrived(floor)
}

pub fn car_door_opened() -> Event {
    Event::DoorUpdate(true)
}

pub fn car_door_closed() -> Event {
    Event::DoorUpdate(false)
}

pub fn lobby_call_button_pressed(floor: i32, direction: Direction) -> Event {
    Event::PressDirectionButton(floor, direction)
}

pub fn car_floor_button_pressed(floor: i32) -> Event {
    Event::PressFloorButton(floor)
}
