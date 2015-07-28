extern crate clap;
extern crate term_painter;
extern crate regex;

use std::io::Read;
use std::io::{Write,Error};
use std::fs;
use error;
use std::collections::btree_map::BTreeMap;
use std::num;

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;

pub fn do_swap(first: u16, second: u16, dir: &str) -> Result<(),error::BookError>{

  let dir_entries = try!(sorted_dir_entries(dir));



  if dir_entries.contains_key(&first) && dir_entries.contains_key(&second) {
    println!("{}  {}_{} with {}_{}",Yellow.bold().paint("Swap"),first,dir_entries.get(&first).unwrap(),second,dir_entries.get(&second).unwrap());
    let first_name = dir_entries.get(&first).unwrap();
    let second_name = dir_entries.get(&second).unwrap();
    let new_name_of_first = format!("{}/{}_{}",&dir,&second,&first_name);
    let old_name_of_first = format!("{}/{}_{}",&dir,&first,&first_name);
    let new_name_of_second = format!("{}/{}_{}",&dir,&first,&second_name);
    let old_name_of_second = format!("{}/{}_{}",&dir,&second,&second_name);
    let backup_name_of_second = format!("{}.{}",old_name_of_second,"bak");
    try!(fs::rename(&old_name_of_second,&backup_name_of_second));
    try!(fs::rename(&old_name_of_first,&new_name_of_first));
    try!(fs::rename(&backup_name_of_second,&new_name_of_second));
    let mut f = try!(fs::File::open(format!("{}/index.adoc",&dir)));
    let mut file_content = String::new();
    try!(f.read_to_string(&mut file_content));
    let re = regex::Regex::new(&*format!(r"({}|{})/index.adoc\[\]",&first_name,&second_name)).unwrap();
    let new_content  = re.replacen(&file_content,2,|caps : &regex::Captures|{
      let include_match = caps.at(0).unwrap_or("");
      if include_match == &*format!("{}/index.adoc[]",&first_name) {
         format!("{}/index.adoc[]",&second_name)
      } else {
        format!("{}/index.adoc[]",&first_name)
      }
    });

    let mut temp_file = try!(fs::File::create(format!("{}/temp_file",&dir)));
    try!(write!(temp_file,"{}",new_content));
    try!(fs::rename(format!("{}/temp_file",&dir),format!("{}/index.adoc",&dir)));
    Ok(())
  } else {
    Err(error::BookError::NormalBookError(format!("can't move section {} to {}",first,second)))
  }
}


pub fn sorted_dir_entries(path: &str) -> Result<BTreeMap<u16,String>,Error> {
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
    Ok(number) => Ok((number,iter.collect::<Vec<&str>>().connect("_"))),

    Err(err) => Err(err)
  }
}
