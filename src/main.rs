use crate::exercise::{Exercise, ExerciseList};
use clap::Parser;
use std::{fs, io};

mod exercise;
mod exercises;
mod generate;
mod run;

/// Program to help solve leetcode exercises
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Exercise name to look for
    #[arg(required_unless_present = "generate")]
    exercise: Option<String>,
    /// Generate new exercise file
    #[arg(short, long, value_name = "FILE")]
    generate: Option<String>,
}

fn main() -> Result<(), io::Error> {
    let toml_str = &fs::read_to_string("exercises.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

    let args = Args::parse();

    if let Some(exercise_name) = args.exercise {
        let exercise = find_exercise(&exercise_name, &exercises);
        run::run(exercise).unwrap_or_else(|_| std::process::exit(1));
    } else if let Some(filename) = args.generate {
        generate::generate_exercise(filename.clone())?;
        println!("Exercise '{}' generated!", filename);
    }

    Ok(())
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    exercises
        .iter()
        .find(|e| e.name == name)
        .unwrap_or_else(|| {
            println!("No exercise found for '{name}'!");
            std::process::exit(1)
        })
}
