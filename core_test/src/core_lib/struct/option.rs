#[allow(dead_code)]
#[cfg(test)]
mod option_tests {
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

    fn maybe_num() -> Option<i32> {
        Some(1)
    }

    fn maybe_word() -> Option<String> {
        Some("str_some".to_owned())
    }

    fn no_num() -> Option<i32> {
        None
    }

    #[test]
    fn map_combinations() {
        let plus_one = match maybe_num() {
            Some(num) => Some(num + 1),
            None => None,
        };
        println!("{:?}", plus_one);

        let plus_one_v2 = maybe_num().map(|num| num + 1);
        println!("{:?}", plus_one_v2);

        let non_num = no_num().map(|num| num + 1);
        println!("{:?}", non_num);

        let word_len = maybe_word().map(|word| word.len());
        println!("{:?}", word_len);

        let word_len_x2 = maybe_word()
            .map(|word| word.len())
            .map(|len| len * 2);
        println!("{:?}", word_len_x2);
    }

    #[test]
    fn options_combinations() {
        // let a = Some(1);
        let a = None;
        dbg!(a);
        let a_is_some = a.is_some();
        dbg!(a_is_some);
        let a_is_none = a.is_none();
        dbg!(a_is_none);
        let a_mapped = a.map(|n| n + 1);
        dbg!(a_mapped);
        let a_filtered = a.filter(|n| n == &1); //borrow comparison
        dbg!(a_filtered);
        let a_or_else = a.or_else(|| Some(5));
        dbg!(a_or_else);
        let a_unwrapped_or_else = a.unwrap_or_else(|| 0);
        dbg!(a_unwrapped_or_else);
    }
}