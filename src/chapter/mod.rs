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
  InScope(String,u8),
  OutOfScope
}

pub fn chapter(arguments: &ArgMatches) -> Result<(),error::BookError> {
  let name = arguments.value_of("name").unwrap();//safe. name is required argument.
  match find_content_root() {
      Location::InScope(path,level) => add_part(name,&*path,level),
      Location::OutOfScope => Err(error::BookError{message : "You are not within a project directory"})
  }
}

fn add_part(title: &str,path: &str,level : u8) -> Result<(),error::BookError> {

    Ok(())
}


fn find_last_number(path: &str) -> u16 {
    
}


fn find_content_root() -> Location {
    let curret_dir = match env::current_dir(){
        Ok(dir) => dir,
        Err(err) => return Location::OutOfScope
    };
    let p = curret_dir.to_str().unwrap();
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
