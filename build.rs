use std::env;
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub struct Cmake {
    path: PathBuf,
}

impl Cmake {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: env::current_dir().unwrap().join(path),
        }
    }

    fn print_output(mut command: Child) {
        let stdout = command.stdout.as_mut().unwrap();
        let reader = BufReader::new(stdout);
        let lines = reader.lines();

        for l in lines {
            println!("{}", l.unwrap());
        }

        let exit_status = command.wait().unwrap().code().unwrap();

        println!("Exited with status {}", exit_status);
    }

    fn generate(&self) -> PathBuf {
        let build_path = env::current_dir().unwrap().join(Path::new("libs"));

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

    pub fn build(&self) -> PathBuf {
        let build_path = self.generate();

        let command = Command::new("cmake")
            .arg("--build")
            .arg(build_path.clone())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        Self::print_output(command);

        build_path
    }
}

fn main() {
    let target_os = env::var("TARGET").unwrap();
    let sdl_path = Cmake::new("SDL").build();

    println!("cargo:rustc-link-search={}", sdl_path.to_str().unwrap());

    println!("cargo:rustc-link-lib=static=SDL2");

    if target_os.contains("darwin") {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Carbon");
        println!("cargo:rustc-link-lib=framework=ForceFeedback");
        println!("cargo:rustc-link-lib=framework=GameController");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=iconv");
    }
}
