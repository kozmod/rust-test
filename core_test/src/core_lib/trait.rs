#[allow(dead_code)]
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

    #[derive(Debug, Copy, Clone)]
    struct Package {
        weight: i64,
    }

    impl Package {
        fn new(w: i64) -> Self {
            Self { weight: w }
        }
    }

    impl Default for Package {
        fn default() -> Self {
            Self { weight: 3 }
        }
    }

    #[test]
    fn default_impl() {
        let mut p  = Package::new(99);
        dbg!(p);
        p  = Package::default();
        dbg!(p);
    }
}