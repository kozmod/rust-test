#[allow(dead_code)]
#[cfg(test)]
mod tests {

    #[test]
    fn vec() {
        let mut v  = vec![1,2,3];
        v.push(4);
        v.push(4);
        println!("{:?}",v);
        println!("{:?}",v.len());
        println!("{:?}",v.pop());
        println!("{:?}",v);
        println!("{:?}",v[0]);
        for num in v {
            println!("{:?}",num);
        }
    }
    #[test]
    fn new() {
        let mut v  = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        v.push(4);
        println!("{:?}",v);
        println!("{:?}",v.len());
        println!("{:?}",v.pop());
        println!("{:?}",v);
        println!("{:?}",v[0]);
        for num in v {
            println!("{:?}",num);
        }
    }

    #[derive(Debug)]
    struct Some {
        i_val: i32,
    }

    #[test]
    fn struct_vec() {
        let  v  = vec![
            Some{i_val:33},
            Some{i_val:55},
            Some{i_val:99},
        ];

        for some in v {
            println!("{:?}",some);
        }
    }
}