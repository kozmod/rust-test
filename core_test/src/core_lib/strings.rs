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
}