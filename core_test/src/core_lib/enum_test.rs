#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[derive(Debug)]
    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
        CUSTOM(i32)
    }

    #[test]
    fn match_enum() {
        let some_enum = Direction::UP;
        match some_enum {
            Direction::UP => println!("Up"),
            Direction::DOWN => println!("Down"),
            Direction::LEFT => println!("Left"),
            Direction::RIGHT => println!("Right"),
            _ => println!("Other"),
        }
    }

    #[test]
    fn match_enum_custom() {
        let some_enum = Direction::CUSTOM(33);
        match some_enum {
            Direction::CUSTOM(11) => println!("c11"),
            Direction::CUSTOM(33) => println!("c33"),
            Direction::UP => println!("Up"),
            Direction::DOWN => println!("Down"),
            Direction::LEFT => println!("Left"),
            Direction::RIGHT => println!("Right"),
            _ => println!("Other"),
        }
    }


}