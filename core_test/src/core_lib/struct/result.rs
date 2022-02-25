#[cfg(test)]
mod tests {

    fn get_result(i:i32) -> Result<String, String> {
        match i {
            1 => Ok("ok".to_owned()),
            2 => Ok("ok - 2".to_owned()),
            _ => Err("some error".to_owned())
        }
    }

    fn get_result_v2(i:i32) -> Result<(), String> {
        let res:String  = get_result(i)?; // look at "?"
        println!("ok result {:?}", res);
        Ok(())
    }

    #[test]
    fn match_result() {
        let res = get_result(99);
        match res {
            Ok(_) => println!("just ok"),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn match_result_v2() {
        match  get_result_v2(3) {
            Ok(_) => println!("just ok"),
            Err(e) => println!("{:?}", e),
        }
    }
}