#[allow(dead_code)]
#[cfg(test)]
mod tests {
    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    #[test]
    fn match_enum() {
        let some_enum = Direction::UP;
        match some_enum {
            Direction::UP => println!("Up"),
            Direction::DOWN => println!("Down"),
            Direction::LEFT => println!("Left"),
            Direction::RIGHT => println!("Right"),
        }
    }
}