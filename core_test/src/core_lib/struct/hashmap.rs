#[allow(dead_code)]
#[cfg(test)]
mod hashmap_test {
    use std::collections::HashMap;

    #[test]
    fn raw() {
        let mut map = HashMap::new();
        map.insert("a",1);
        map.insert("a",2);
        map.insert("b",3);
        println!("{:?}", map.get("a"));

        map.remove("a");
        println!("{:?}", map.get("a"));

        map.insert("a",4);

        for (key, val) in map.iter() {
            println!("{:?}:{:?}", key, val);
        }
        for key in map.keys() {
            println!("{:?}", key);
        }
        for val in map.values() {
            println!("{:?}", val);
        }
    }

}