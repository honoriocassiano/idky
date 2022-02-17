use std::{env, fs};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use crate::Shader::{Fragment, Vertex};

enum Shader {
    Vertex(PathBuf),
    Fragment(PathBuf),
}

fn get_shaders<T: AsRef<Path>>(shaders_path: T) -> Vec<Shader> {
    let dir = fs::read_dir(shaders_path).unwrap();

    dir.filter_map(|f| {
        let f = f.unwrap();

        if !f.metadata().unwrap().is_file() {
            return None;
        }

        let path = f.path();

        match path.extension() {
            None => None,
            Some(extension) => match extension.to_ascii_lowercase().to_str().unwrap() {
                "vert" => Some(Vertex(path)),
                "frag" => Some(Fragment(path)),
                _ => None,
            },
        }
    })
    .collect()
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

fn main() {
    let shaders_path = Path::new("shaders").canonicalize().unwrap();

    // TODO Run on Windows to check if the .exe extension will be necessary
    let glslc = Path::new(env::var("VULKAN_SDK").unwrap().as_str()).join("bin/glslc");

    let shader_files = get_shaders(shaders_path.clone());

    shader_files.iter().for_each(|f| {

        let suffix = match f {
            Vertex(_) => { "_vert.spv" }
            Fragment(_) => { "_frag.spv" }
        };

        let (old_filename, new_filename) = match f {
            Vertex(filename) |
            Fragment(filename) => {
                let old_filename = filename.to_str().unwrap().to_owned();

                let prefix = filename
                    .file_stem()
                    .map(|fp| fp.to_str())
                    .flatten()
                    .unwrap();

                (old_filename, format!("{}{}", prefix, suffix))
            }
        };

        let old_filename = shaders_path.join(old_filename);
        let new_filename = shaders_path.join(new_filename);

        let child = Command::new(glslc.as_os_str())
            .arg(old_filename.clone())
            .arg("-o")
            .arg(new_filename)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        print_output(child);

        println!("cargo:rerun-if-changed={}", old_filename.display());
    });

    println!("cargo:rerun-if-changed=build.rs");
}
