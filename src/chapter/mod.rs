extern crate clap;
use self::clap::ArgMatches;
use std::io::Error;
use std::os;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
use error;



enum Location {
  Root,
  Chapter,
  Section,
  SubSection,
  SubSubSection,
  OutOfScope
}

pub fn chapter(arguments: &ArgMatches) -> Result<(),error::Error> {
  let name = arguments.value_of("name").unwrap();//safe. name is required argument.
  let location = check_location(".".to_string(),0);

  if let  Location::OutOfScope = location {
    return error::Error{message : "You are not within a project directory".to_string()};
  }
  Ok(())
}

fn check_location(path: String,depth: u8) -> Location {
  let curr_path = format!("{}.git/description",path);
  let mut f = File::open(&*curr_path);
  let mut file_content = String::new();
  if f.is_ok() {
    f.ok().unwrap().read_to_string(&mut file_content);
    let content = file_content.split('_').last();
    if content.is_some() && content.unwrap() == "book" {
        return convert_to_location(depth);
    }
  }
  match Path::new(&*path).parent() {
    Some(p) => check_location(format!("../{}",path),depth+1),
    None => return Location::OutOfScope
  }
}

fn convert_to_location(depth:u8) -> Location {
  match depth {
    0 => Location::Root,
    1 => Location::Chapter,
    2 => Location::Section,
    3 => Location::SubSection,
    _ => Location::SubSubSection,
  }
}
