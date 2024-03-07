use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
};

const EXERCISE_TEMPLATE: &str = "
[[exercises]]
name = \"{}\"
path = \"src/exercises/{}.rs\"
mode = \"test\"";

const EXERCISE_MOD_TEMPLATE: &str = "mod {};\n";

pub fn generate_exercise(filename: String) -> Result<(), io::Error> {
    let mut file = File::open("template.txt")?;
    let mut template_contents = String::new();
    file.read_to_string(&mut template_contents)?;

    let mut exercise_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(format!("src/exercises/{}.rs", filename))?;
    exercise_file.write(template_contents.as_bytes())?;

    let mut exercises_toml = OpenOptions::new().append(true).open("exercises.toml")?;

    let exercise_content = String::from(EXERCISE_TEMPLATE).replace("{}", &filename);
    exercises_toml.write(exercise_content.as_bytes())?;

    let exercise_mod_content = String::from(EXERCISE_MOD_TEMPLATE).replace("{}", &filename);
    let mut exercise_mod_file = OpenOptions::new()
        .append(true)
        .open("src/exercises/mod.rs")?;
    exercise_mod_file.write(exercise_mod_content.as_bytes())?;

    Ok(())
}
