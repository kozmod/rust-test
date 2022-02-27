#[cfg(test)]
mod range_tests {

    #[test]
    fn simple() {
        let range_1_3 =  1..=3;
        let range_1_2 =  1..3;
        dbg!(&range_1_3);
        dbg!(&range_1_2);
        for n in range_1_3 {
            println!("{:?}", n)
        }
        println!("---");
        for n in range_1_2 {
            println!("{:?}", n)
        }
    }

    #[test]
    fn char() {
        for n in 'a'..='f' {
            println!("{:?}", n)
        }
    }

}