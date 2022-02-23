#[cfg(test)]
mod tests {
    #[test]
    fn string_concat() {
        let x = "hello";
        let res: String = x.chars()
            .enumerate()
            .fold(String::new(), |res, (i, ch)| {
                res + &format!("{} {}\n", i, ch)
            });

        println!("{}", res);
    }

    fn print_it(data: &str) {
        println!("{:?}", data);
    }

    #[test]
    fn strings() {
        print_it("string slice");

        let owned_str = "owned string".to_owned();
        print_it(&owned_str);

        let other_owned_str = String::from("other owned string");
        print_it(&other_owned_str);
    }

}