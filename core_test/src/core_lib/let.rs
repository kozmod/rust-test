#[allow(dead_code)]
#[cfg(test)]
mod let_tests {
    #[test]
    fn simple_if_let_else() {
        let maybe_user = Some("Jerry");
        match maybe_user {
            Some(user) => println!("{:?}", user),
            None => println!("no user!"),
        }

        if let Some(user) = maybe_user {
            println!("user={:?}", user);
        } else {
            println!("no user! :-(");
        }
    }

    enum Color {
        Red,
        Yellow,
    }

    #[test]
    fn enumeration_if_let_else() {
        if let Color::Red = Color::Red {
            println!("color is Red");
        } else {
            println!("wat is the color?!");
        }
    }

    #[test]
    fn simple_while_let() {
        let mut data = Some(3);
        while let Some(_) = data {
            println!("loop");
            data = None;
        }
        println!("done");
    }

    #[test]
    fn simple_let_with_iterator() {
        let nums = vec![1,2,3];
        let mut itr = nums.iter();
        while let Some(num) = itr.next() {
            println!("{:?}", num);
        }
        println!("done");
    }
}