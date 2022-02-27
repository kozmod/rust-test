#[cfg(test)]
mod trait_tests {
    trait Fall {
        fn hit_the_ground(&self);
        fn ha(&self);
    }

    trait Up {
        fn stand_up(&self);
    }

    // trait T: Fall + Up {}

    #[derive(Debug, Copy, Clone)]
    struct Some;

    impl Fall for Some {
        fn hit_the_ground(&self) {
            println!("the some broke!")
        }

        fn ha(&self) {
            println!("HA HA HA")
        }
    }

    impl Up for Some {
        fn stand_up(&self) {
            println!("STAND UP!")
        }
    }

    fn fall(i: impl Fall) {
        i.hit_the_ground();
        println!(":-)");
        i.ha();
    }

    fn stand_up(i: impl Up) {
        i.stand_up();
    }


    #[test]
    fn simple_impl() {
        let s = Some {};
        fall(s);
        stand_up(s);
    }
}