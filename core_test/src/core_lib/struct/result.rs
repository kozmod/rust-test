#[allow(dead_code)]
#[cfg(test)]
mod result_tests_v1 {

    fn get_result(i: i32) -> Result<String, String> {
        match i {
            1 => Ok("ok".to_owned()),
            2 => Ok("ok - 2".to_owned()),
            _ => Err("some error".to_owned())
        }
    }

    fn get_result_v2(i: i32) -> Result<(), String> {
        let res: String = get_result(i)?; // look at "?"
        println!("ok result {:?}", res);
        Ok(())
    }

    #[test]
    fn match_result() {
        let res = get_result(99);
        match res {
            Ok(_) => println!("just ok"),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn match_result_v2() {
        match get_result_v2(3) {
            Ok(_) => println!("just ok"),
            Err(e) => println!("{:?}", e),
        }
    }

    #[derive(Debug)]
    enum Status {
        Terminated,
        Process,
        Wait,
    }

    struct Job {
        id: i32,
        status: Status,
    }

    fn try_status(job: &Job) -> Result<(), String> {
        match job.status {
            Status::Terminated => Err("terminated".to_owned()),
            Status::Process => Err("process".to_owned()),
            Status::Wait => Ok(()),
        }
    }

    fn print_status(job: &Job) -> Result<(), String> {
        let status = try_status(job)?;
        println!("{:?}", status);
        Ok(())
    }

    #[allow(unused_must_use)]
    #[test]
    fn result_question_mark() {
       let job = Job {
           id: 1,
           status: Status::Wait,
       };
        print_status(&job);
    }
}