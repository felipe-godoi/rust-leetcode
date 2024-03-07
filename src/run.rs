use crate::exercise::{Exercise, Mode};

pub fn run(exercise: &Exercise) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => compile_and_test(exercise)?,
        Mode::Compile => compile_and_run(exercise)?,
    }
    Ok(())
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
