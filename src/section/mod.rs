extern crate clap;
extern crate term_painter;

use self::clap::ArgMatches;
use std::os;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
use std::io::{Write,Error};
use error;
use self::term_painter::{ToStyle};
use self::term_painter::Color::*;
use std::fs::OpenOptions;



enum Location {
  InScope(String,u8),
  OutOfScope
}

pub fn section(arguments: &ArgMatches) -> Result<(),error::BookError> {
  let curret_dir = try!(env::current_dir());
  let p = curret_dir.to_str().unwrap();
  let name = arguments.value_of("name").unwrap();//safe. name is required argument.
  match find_content_root(p) {
      Location::InScope(path,level) => add_part(name,p,level),
      Location::OutOfScope => Err(error::BookError::NormalBookError("not within project directory."))
  }
}

fn add_part(title: &str,path: &str,level : u8) -> Result<(),error::BookError> {
  let new_number = try!(find_last_number(path))+1;
  create_dir!(path,&*format!("{}_{}",new_number,title));
  let mut headings = "=".to_string();
  let mut options_include = "include::../".to_string();
  for _i in 0 .. level {
    headings.push_str("=");
    options_include.push_str("../")
  }
  options_include.push_str("includes/config.adoc[]\n");
  create_file!(path,&*format!("{}_{}/index.adoc",new_number,title),
"{} {}
{}
",headings,title,options_include);
  append_file!("index.adoc","
include::{}/index.adoc[]
",&*format!("{}_{}",new_number,title));
  Ok(())
}


fn find_last_number(path: &str) -> Result<u16,Error> {
  let mut highest_number = 0;
  for entry in try!(fs::read_dir(path)) {
    let dir = try!(entry);
    if dir.file_type().unwrap().is_dir() {
      let raw_file_name =  dir.file_name();
      let file_name = raw_file_name.to_str().unwrap();
      if file_name.contains("_") {
        let first_parts: Vec<&str> = file_name.split("_").collect();
        if let Ok(number) = first_parts[0].parse::<u16>() {
          highest_number = if number > highest_number {
            number
          } else {
            highest_number
          };
        }
      }
    }
  }
  Ok(highest_number)
}


fn find_content_root(p: &str) -> Location {
    println!("splitting the following path: {}",p);
    let mut buff = "/".to_string();

    let file_name = Path::new(p).file_name().unwrap().to_str().unwrap();
    let (possible_root, mut depth) = if file_name == "content" {
        (p.to_string(),1)
    } else {
        let mut parts : Vec<&str> = p.split("/content/").collect();
        if parts.len() >= 1 {
            let last_bits: Vec<&str> = parts.last().unwrap().split("/").collect();
            (format!("{}/content",parts[0]),(last_bits.len() as u8)+1)
        } else {
            return Location::OutOfScope;
        }
    };

    let project_dir = Path::new(&*possible_root).parent().unwrap();
    let mut f = File::open(project_dir.join(".git/description"));
    let mut file_content = String::new();
    if f.is_ok() {
      f.ok().unwrap().read_to_string(&mut file_content);
      let content = file_content.split('_').last();
      if content.is_some() && content.unwrap() == "book" {
          return Location::InScope(possible_root.to_string(),depth);
      }
    }
    Location::OutOfScope
}
