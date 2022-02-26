#[allow(dead_code)]
#[cfg(test)]
mod iterator_tests {
    #[test]
    fn vec() {
        let v = vec![1, 2, 3];
        let plus_one: Vec<_> = v.iter()
            .map(|n| n + 1)
            .collect();
        dbg!(plus_one);

        let filtered_plus_two: Vec<i32> = v.iter()
            .filter(|n| n > &&1)
            .map(|n| n + 2)
            .collect();
        dbg!(filtered_plus_two);

        let find_3: Option<&i32> = v.iter()
            .find(|n| n == &&3);
        dbg!(find_3);

        let count = v.iter()
            .count();
        dbg!(count);

        let last = v.iter()
            .last();
        dbg!(last);

        let take_first_2: Vec<_> = v.iter()
            .take(2)
            .collect();
        dbg!(take_first_2);
    }
}