use std::env;
use std::{fs::File, io::Write, process::Command};

//https://rust-lang-nursery.github.io/rust-cookbook/os/external.html#run-an-external-command-passing-it-stdin-and-check-for-an-error-code

// https://github.com/rust-lang/rust/issues/23344#issuecomment-164950238
// https://github.com/rust-lang/rust/blob/master/library/std/src/process/tests.rs
// https://github.com/rust-lang/rust/blob/master/library/std/src/process/tests.rs#L71
// https://github.com/rust-lang/rust/blob/master/library/std/src/process/tests.rs#L125
// window 한글깨짐: 레지스터리 수정 949->65001 ( 사용자, 시스템 모두)

#[derive(Debug)]
pub struct CMD {
    pub args: Vec<String>,
}
impl CMD {
    pub fn run(&self) {
        self.run_output();
    }

    fn get_args(&self) -> (String, Vec<String>) {
        let program = if cfg!(target_os = "windows") {
            "bash".to_string()
        } else {
            "sh".to_string()
        };
        let mut args = vec!["-C".to_owned()];
        let mut a2 = self.args.clone();
        args.append(&mut a2);
        (program, args)
    }

    fn run_output(&self) {
        let (program, args) = self.get_args();
        println!("program={}, args={:#?}", &program, &args);
        let (mut f_out, mut f_err) = self.get_log_file();
        let mut cmd = Command::new(program)
            .args(args)
            .stdout(f_out)
            .spawn()
            .unwrap();
        cmd.wait().expect("failed to finish echo");
    }

    fn get_log_file(&self) -> (File, File) {
        let (f_out, ferr): (String, String) =
            (format!("{}.log", "test-1"), format!("{}-err.log", "test"));
        (File::create(f_out).unwrap(), File::create(ferr).unwrap())
    }
}
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works231321() {
        // CMD {
        //     run_file: "dir".to_string(),
        // }
        // .run();

        // assert_eq!(2, 4);
    }
}
