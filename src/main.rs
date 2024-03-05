use std::fs;

use exercise::Mode;

use crate::exercise::{Exercise, ExerciseList};

mod exercise;
mod exercises;

fn main() {
    let toml_str = &fs::read_to_string("exercises.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

    let exercise_name = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No exercise name provided!");
        std::process::exit(1)
    });

    let exercise = find_exercise(&exercise_name, &exercises);
    run(exercise).unwrap_or_else(|_| std::process::exit(1));
}

fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            println!("stderr: {}", output.stderr);
            return Err(());
        }
    };

    let result = compilation.run();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            Err(())
        }
    }
}

fn compile_and_test(exercise: &Exercise) -> Result<(), ()> {
    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            println!("stderr: {}", output.stderr);
            return Err(());
        }
    };

    let result = compilation.run();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            Err(())
        }
    }
}

fn run(exercise: &Exercise) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => compile_and_test(exercise)?,
        Mode::Compile => compile_and_run(exercise)?,
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
