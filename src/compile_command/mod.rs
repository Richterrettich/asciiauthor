use std::process::Command;
use error;
use util;


pub fn compile(dir: &str, backend: &str) -> Result<(), error::BookError> {


    match util::find_content_root(dir) {
        util::Location::InScope(path, _level) => execute_compilation(&path, backend),
        util::Location::OutOfScope => {
            Err(error::BookError::NormalBookError("not within project directory.".to_string()))
        }
    }


}


fn execute_compilation(dir: &str, backend: &str) -> Result<(), error::BookError> {
    let possible_output = Command::new(backend).arg(format!("{}/index.adoc", dir)).output();
    match possible_output {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(_) => {
            return Err(error::BookError::NormalBookError("Could not compile project.".to_string()))
        }
    }
    Ok(())
}
