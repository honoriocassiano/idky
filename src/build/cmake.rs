use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

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
        let command = Command::new("cmake")
            .arg("-S")
            .arg(self.path.as_os_str())
            .arg("-B")
            .arg(env::current_dir().unwrap().join(Path::new("build")))
            .output();

        match command {
            Ok(out) => {
                let x: String = out.stdout.iter()
                    .map(|&c| c as char)
                    .collect();


                println!("{}", x);

                Ok(())
            }
            Err(err) => {
                println!("{}", err);

                Err(err.to_string())
            }
        }
    }
}
