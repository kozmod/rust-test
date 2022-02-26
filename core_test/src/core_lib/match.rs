#[allow(dead_code)]
#[cfg(test)]
mod match_tests {
    #[derive(Debug)]
    struct Data {
        val: String,
        i: i32,
    }

    #[test]
    fn match_struct() {
        let data = Data {
            i: 33,
            val: "some str".to_owned(),
        };
        match data {
            Data { i, .. } => println!("data 'i' is {:?}", i)
        };

        match data {
            Data { i: 11, val } => println!("data 'v' is {:?}", val),
            Data { i: 33, .. } => println!("data 'i' is {:?}", data.i),
            _ => {}
        }
    }
}