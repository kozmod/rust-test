#[allow(dead_code)]
#[cfg(test)]
mod struct_tests {
    use core::fmt;

    #[derive(Debug)]
    enum Direction {
        UP,
        DOWN,
    }
    impl fmt::Display for Direction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    struct Some {
        i_val: i32,
        str_val: String,
        d_val: Direction,
    }

    #[test]
    fn struct_test() {
        let some = Some {
            i_val: 33,
            str_val: "hello".to_string(),
            d_val: Direction::DOWN,
        };
        println!("{:?} - {:?} - {:?}", some.i_val, some.str_val, some.d_val);
    }
}