extern crate term_painter;

use std::path::Path;
use swap_command;
use error;
use move_command;
use std::collections::BTreeMap;
use std::fs;
use std::iter::Iterator;
use std::io::{Read,Write,Error};
use self::term_painter::{ToStyle};
use self::term_painter::Color::*;


pub fn do_remove(number: u16,base: &str) -> Result<(),error::BookError> {
  let dir_entries = try!(swap_command::sorted_dir_entries(base));
  let dir_name = format!("{}_{}",number,dir_entries.get(&number).unwrap());
  remove_dir!(base,dir_name);
  println!("done removing");
  let highest_value = move_command::get_last_section_number(&dir_entries);
  try!(move_command::move_section_dirs(number,highest_value,base,&dir_entries));
  println!("done moving");
  try!(move_command::reallocate_index(number,highest_value,base,&dir_entries,|first_part,new_values,last_part|{
    format!("{}{}{}",first_part,new_values,last_part)//,highest_value,dir_entries.get(&highest_value).unwrap())
  }));
  Ok(())
}
