use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
    process::{self, Command},
};

use serde::Deserialize;

#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    format!("./temp_{}_{thread_id}", process::id())
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct ExerciseOutput {
    pub stdout: String,
    pub stderr: String,
}

impl Exercise {
    pub fn compile(&self) -> Result<&Exercise, ExerciseOutput> {
        let cmd = Command::new("rustc")
            .args([self.path.to_str().unwrap(), "-o", &temp_file()])
            .output()
            .expect("Failed to compile the exercise");

        if cmd.status.success() {
            Ok(self)
        } else {
            Err(ExerciseOutput {
                stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
                stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
            })
        }
    }

    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        let cmd = Command::new(temp_file())
            .output()
            .expect("Failed to run 'run' command");

        let output = ExerciseOutput {
            stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
        };

        if cmd.status.success() {
            Ok(output)
        } else {
            Err(output)
        }
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

mod exercise {}
