use std::env;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub struct Cmake {
    path: PathBuf,
}

impl Cmake {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: env::current_dir().unwrap().join(path)
        }
    }

    fn print_output(mut command: Child) {
        let stdout = command.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        for l in lines {
            println!("{}", l.unwrap());
        }

        let exit_status = command.wait()
            .unwrap()
            .code()
            .unwrap();

        println!("Exited with status {}", exit_status);
    }

    fn generate(&self) -> PathBuf {
        let build_path = env::current_dir().unwrap().join(Path::new("build"));

        let command = Command::new("cmake")
            .arg("-S")
            .arg(self.path.as_os_str())
            .arg("-B")
            .arg(build_path.clone())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        Self::print_output(command);

        build_path
    }

    pub fn build(&self) {
        let build_path = self.generate();

        let command = Command::new("cmake")
            .arg("--build")
            .arg(build_path)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        Self::print_output(command);
    }
}
