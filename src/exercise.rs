use std::{
    fmt::{self, Display, Formatter},
    fs::remove_file,
    path::PathBuf,
    process::{self, Command},
};

use serde::Deserialize;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_EDITION_ARGS: &[&str] = &["--edition", "2021"];

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

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
}

#[derive(Debug, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub mode: Mode,
}

#[derive(Debug)]
pub struct ExerciseOutput {
    pub stdout: String,
    pub stderr: String,
}

pub struct CompiledExercise<'a> {
    exercise: &'a Exercise,
    _handle: FileHandle,
}

impl<'a> CompiledExercise<'a> {
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        self.exercise.run()
    }
}

struct FileHandle;

impl Drop for FileHandle {
    fn drop(&mut self) {
        clean();
    }
}

#[inline]
fn clean() {
    let _ignored = remove_file(temp_file());
}

impl Exercise {
    pub fn compile(&self) -> Result<CompiledExercise, ExerciseOutput> {
        let cmd = match self.mode {
            Mode::Compile => Command::new("rustc")
                .args([self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .output(),
            Mode::Test => Command::new("rustc")
                .args(["--test", self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .output(),
        }
        .expect("Failed to compile the exercise");

        if cmd.status.success() {
            Ok(CompiledExercise {
                exercise: self,
                _handle: FileHandle,
            })
        } else {
            clean();

            Err(ExerciseOutput {
                stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
                stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
            })
        }
    }

    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        let arg = match self.mode {
            Mode::Test => "--show-output",
            _ => "",
        };

        let cmd = Command::new(temp_file())
            .arg(arg)
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
