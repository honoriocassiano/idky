use std::env;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct Cmake {
    path: PathBuf,
}

impl Cmake {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: env::current_dir().unwrap().join(path)
        }
    }

    pub fn build(&self) -> Result<(), String> {
        let mut command = Command::new("cmake")
            .arg("-S")
            .arg(self.path.as_os_str())
            .arg("-B")
            .arg(env::current_dir().unwrap().join(Path::new("build")))
            .stdout(Stdio::piped())
            .spawn()
            // TODO Improve error handling
            .unwrap();

        // TODO Improve error handling
        let stdout = command.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        for l in lines {
            // TODO Improve error handling
            println!("{}", l.unwrap());
        }

        match command.wait() {
            Ok(status) => {
                // TODO Improve error handling
                println!("Exited with status {}", status.code().unwrap());
                Ok(())
            }
            Err(_) => {
                // TODO Improve error handling
                println!("Failed to run cmake");
                Err("Error".to_owned())
            }
        }
    }
}
