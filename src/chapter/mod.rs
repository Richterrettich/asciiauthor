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



enum Location {
  InScope(String,u8),
  OutOfScope
}

pub fn chapter(arguments: &ArgMatches) -> Result<(),error::BookError> {
  let curret_dir = try!(env::current_dir());
  let p = curret_dir.to_str().unwrap();
  let name = arguments.value_of("name").unwrap();//safe. name is required argument.
  match find_content_root(p) {
      Location::InScope(path,level) => add_part(name,p,level),
      Location::OutOfScope => Err(error::BookError::NormalBookError("not within project directory."))
  }
}

fn add_part(title: &str,path: &str,level : u8) -> Result<(),error::BookError> {
  let new_number = try!(find_last_number(path));
  create_dir!(path,&*format!("{}_{}",new_number,title));
  let mut headings = "=".to_string();
  //include::../includes/config.adoc[]
  let mut options_include = "include::".to_string();
  for _i in 0 .. level {
    headings.push_str("=");
    options_include.push_str("../")
  }
  options_include.push_str("includes/options.adoc[]");
  create_file!(path,&*format!("{}_{}/index.adoc",new_number,title),
"{} {}
{}
",headings,title,options_include);
  
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
        let first_parts: Vec<&str> = file_name.split(".").collect();
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
    let mut buff = "/".to_string();
    let mut root = Location::OutOfScope;
    let parts : Vec<&str> = p.split("/content/").collect();
    if parts.len() >= 1 {
        let possible_root = format!("{}/.git/description",parts[0]);
        let mut f = File::open(&*possible_root);
        let mut file_content = String::new();
        if f.is_ok() {
          f.ok().unwrap().read_to_string(&mut file_content);
          let content = file_content.split('_').last();
          if content.is_some() && content.unwrap() == "book" {
              if parts.len() == 1 {
                  root = Location::InScope(possible_root,0);
              } else {
                  let last_bits: Vec<&str> = parts.last().unwrap().split("/").collect();
                  root = Location::InScope(possible_root,last_bits.len() as u8);
              }
          }
        }
    }
    root
}

fn check_location(path: String,depth: u8) -> Location {
  let curr_path = format!("{}/../.git/description",path);
  let mut f = File::open(&*curr_path);
  let mut file_content = String::new();
  if f.is_ok() {
    f.ok().unwrap().read_to_string(&mut file_content);
    let content = file_content.split('_').last();
    if content.is_some() && content.unwrap() == "book" {
        return Location::InScope(path,depth);
    }
  }
  match Path::new(&*path).parent() {
    Some(p) => check_location(format!("../{}",path),depth+1),
    None => return Location::OutOfScope
  }
}
