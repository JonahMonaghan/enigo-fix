use enigo::{Enigo, Key, Keyboard, Direction, Settings};

fn release_all_keys(enigo: &mut Enigo) {
    enigo.key(Key::Return, Direction::Release).unwrap();
    enigo.key(Key::Unicode('z'), Direction::Release).unwrap();
    enigo.key(Key::Unicode('x'), Direction::Release).unwrap();
    enigo.key(Key::UpArrow, Direction::Release).unwrap();
    enigo.key(Key::RightArrow, Direction::Release).unwrap();
    enigo.key(Key::DownArrow, Direction::Release).unwrap();
    enigo.key(Key::LeftArrow, Direction::Release).unwrap();
}

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    release_all_keys(&mut enigo);
}