#[cfg(test)]
mod closure_tests {
    fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
        op(a, b)
    }

    //noinspection ALL
    #[test]
    fn simple() {
        let add = |a, b| a + b;
        let add_pint = |a, b| {
            println!("{:?} and {:?}", a, b);
            a + b
        };
        let sub = |a, b| a - b;
        let mult = |a, b| a * b;
        println!("{}", math(3, 2, Box::new(add)));
        println!("{}", math(3, 2, Box::new(add_pint)));
        println!("{}", math(3, 2, Box::new(sub)));
        println!("{}", math(3, 2, Box::new(mult)));
    }
}