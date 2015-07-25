extern crate clap;
extern crate term_painter;
extern crate regex;

use std::io::Read;
use std::io::{Write,Error,BufReader};
use std::fs;
use self::clap::ArgMatches;
use std::env;
use error;
use std::collections::btree_map::BTreeMap;
use std::num;

pub fn do_move(arguments: &ArgMatches) -> Result<(),error::BookError>{

  let first = value_t_or_exit!(arguments.value_of("old_number"),u16);
  let second = value_t_or_exit!(arguments.value_of("new_number"),u16);

  let dir_entries = try!(sorted_dir_entries());

  if dir_entries.contains_key(&first) && dir_entries.contains_key(&second) {
    let first_name = dir_entries.get(&first).unwrap();
    let second_name = dir_entries.get(&second).unwrap();
    let new_name_of_first = format!("{}_{}",&second,&first_name);
    let old_name_of_first = format!("{}_{}",&first,&first_name);
    let new_name_of_second = format!("{}_{}",&first,&second_name);
    let old_name_of_second = format!("{}_{}",&second,&second_name);

    let backup_name_of_second = format!("{}.{}",old_name_of_second,"bak");

    try!(fs::rename(&old_name_of_second,&backup_name_of_second));
    try!(fs::rename(&old_name_of_first,&new_name_of_first));
    try!(fs::rename(&backup_name_of_second,&new_name_of_second));

    let mut f = try!(fs::File::open("index.adoc"));
    let mut file_content = String::new();
    f.read_to_string(&mut file_content);
    let re = regex::Regex::new(&*format!(r"({}|{})/index.adoc[]",&first_name,&second_name)).unwrap();

    re.replacen(&file_content,2,|caps : &regex::Captures|{
      let include_match = caps.at(0).unwrap_or("");
      if include_match == &*format!("{}/index.adoc[]",&first_name) {
         format!("{}/index.adoc[]",&second_name)
      } else {
        format!("{}/index.adoc[]",&first_name)
      }
    });

    let mut temp_file = try!(fs::File::create("temp_file"));
    try!(write!(temp_file,"{}",file_content));
    try!(fs::rename("temp_file","index.adoc"));
    Ok(())
  } else {
    Err(error::BookError::NormalBookError(format!("can't move section {} to {}",first,second)))
  }
}


fn sorted_dir_entries() -> Result<BTreeMap<u16,String>,Error> {
  let path = try!(env::current_dir());
  let read_dir = try!(fs::read_dir(path));

  let mut result: BTreeMap<u16,String> = BTreeMap::new();

  for entry in read_dir.filter_map(|item| item.ok())
          .filter(|entry| entry.file_type().unwrap().is_dir())
          .map(|dir| dir.file_name())
          .map(|file_name| file_name.to_str().unwrap().to_string())
          .filter(|dir_name| dir_name.contains("_")) {
          if let Ok((number,name)) = extract_number_value(&*entry) {
            result.insert(number,name);
          }
  }
  Ok(result)
}

fn extract_number_value(value:&str) -> Result<(u16,String),num::ParseIntError> {
  let mut iter = value.split('_');
  match iter.next().unwrap().parse::<u16>() {
    Ok(number) => Ok((number,iter.next().unwrap().to_string())),
    Err(err) => Err(err)
  }
}


fn find_dir_by_id(id: &str) -> Option<String> {
  let path = match env::current_dir() {
    Ok(p) => p,
    Err(err) => return None
  };
  let read_dir = match fs::read_dir(path) {
    Ok(r) => r,
    Err(err) => return None
  };
  read_dir.filter_map(|item| item.ok())
          .filter(|entry| entry.file_type().unwrap().is_dir())
          .map(|dir| dir.file_name())
          .map(|file_name| file_name.to_str().unwrap().to_string())
          .filter(|dir_name| dir_name.contains("_"))
          .find(|dir_name| {
            let first_parts: Vec<&str> = dir_name.split("_").collect();
            first_parts[0] == id
          })
          .map(|dir_name| dir_name.to_string())

  /*for entry in try!(fs::read_dir(path)) {
    let dir = try!(entry);
    if dir.file_type().unwrap().is_dir() {
      let raw_file_name =  dir.file_name();
      let file_name = raw_file_name.to_str().unwrap();
      if file_name.contains("_") {
        let first_parts: Vec<&str> = file_name.split("_").collect();
        if first_parts[0] == id {
          match_dir = Some(dir);
          break;
        }
      }
    }
  }
  if let Some(dir) = match_dir {

  }*/

}
