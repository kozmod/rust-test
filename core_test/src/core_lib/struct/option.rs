#[allow(dead_code)]
#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Customer {
        age: i32,
        email: Option<String>,
    }

    #[test]
    fn match_option() {
        let c1 = Customer {
            age: 33,
            email: Some("some@mail.com".to_owned()),
        };

        match c1.email {
            Some(some) => println!("email is {:?}", some),
            None => println!("no email"),
        }
    }
}