#[allow(dead_code)]
#[cfg(test)]
mod expression_tests {

    #[test]
    fn simple_expression() {
        let val  = true;
        let num = if val {
            1.1
        } else {
            9.9
        };
        println!("{:?}", num)
    }
    #[test]
    fn match_expression() {
        let val  = 3;
        let num = match val{
            1 => 1,
            2 => 2,
            _ => 99,
        };
        println!("{:?}", num)
    }
}