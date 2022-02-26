#[cfg(test)]
mod tuples_tests {

    fn one_two_three() -> (i32, i32, i32) {
        (1, 2, 3)
    }

    #[test]
    fn simple_tuple() {
        let   mut nums = one_two_three();
        let (x,y,z) = one_two_three();
        println!("{:?} - {:?}", x, nums.0);
        println!("{:?} - {:?}", y, nums.1);
        println!("{:?} - {:?}", z, nums.2);

        println!("---");
        nums.0 = 33;
        nums.1 = 11;
        nums.2 = 2;

        println!("{:?} - {:?}", x, nums.0);
        println!("{:?} - {:?}", y, nums.1);
        println!("{:?} - {:?}", z, nums.2);

    }
}