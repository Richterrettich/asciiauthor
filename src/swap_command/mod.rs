extern crate clap;
extern crate term_painter;
extern crate regex;

use std::fs;
use error;
use util;

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;

pub fn do_swap(mut first: usize, mut second: usize, base: &str) -> Result<(),error::BookError>{

  if first >= 1 {
    first = first - 1;
  }
  if second >= 1 {
    second = second - 1;
  }

  let mut dir_entries = try!(util::sorted_dir_entries(base));

  if second >= dir_entries.len() {
    second = dir_entries.len()-1;
  }
  if first >= dir_entries.len() {
    first = dir_entries.len()-1;
  }

  if dir_entries.len() >= first && dir_entries.len() >=second {
    println!("{}  {} with {}",Yellow.bold().paint("Swap"),dir_entries[first].to_string(),dir_entries[second].to_string());
    let old_name_of_first = util::dir(base,first,&dir_entries);
    let old_name_of_second = util::dir(base,second,&dir_entries);
    let new_name_of_first = util::assemble_dir_name(base,dir_entries[second].position,&*dir_entries[first].name);
    let new_name_of_second  = util::assemble_dir_name(base,dir_entries[first].position,&*dir_entries[second].name);
    let old_position_of_first = dir_entries[first].position;
    dir_entries[first].position =  dir_entries[second].position;
    dir_entries[second].position = old_position_of_first;
    println!("rename {} to {}",old_name_of_second,new_name_of_second);
    try!(fs::rename(&old_name_of_second,new_name_of_second));
    println!("rename {} to {}",old_name_of_first,new_name_of_first);
    try!(fs::rename(&old_name_of_first,new_name_of_first));
    try!(util::rewrite_index(&mut dir_entries,base));
    try!(util::rewrite_sections(&mut dir_entries,base));
    Ok(())
  } else {
    Err(error::BookError::NormalBookError(format!("can't move section {} to {}",first,second)))
  }
}
