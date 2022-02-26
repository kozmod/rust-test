#[allow(dead_code)]
#[cfg(test)]
mod derive_tests {
    #[derive(Debug, Clone, Copy)]
    enum Direction {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    #[derive(Debug, Clone, Copy)]
    struct Data {
        val: i32,
        direction: Direction,
    }

    // d is owned in this function (copy)
    fn print_data(d: Data){
        println!("{:?}", d)
    }

    #[test]
    fn print() {
        let data = Data {
            val: 1,
            direction: Direction::UP,
        };
        print_data(data);
        print_data(data);
    }
}