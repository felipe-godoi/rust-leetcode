use std::fs;

use crate::exercise::{Exercise, ExerciseList};

mod exercise;

fn main() {
    let toml_str = &fs::read_to_string("exercises.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;

    let exercise_name = std::env::args().nth(1).unwrap_or_else(|| {
        println!("No exercise name provided!");
        std::process::exit(1)
    });

    let exercise = find_exercise(&exercise_name, &exercises);
    let compiled = exercise.compile();

    if let Err(output) = compiled {
        println!("Failed to compile the exercise: {}", output.stderr);
        std::process::exit(1);
    } else if let Ok(exercise) = compiled {
        let output = exercise.run();

        match output {
            Ok(output) => println!("stdout: {}", output.stdout),
            Err(output) => println!("stderr: {}", output.stderr),
        }
    }
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
