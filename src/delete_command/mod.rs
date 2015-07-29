extern crate term_painter;

use util;
use error;
use std::fs;
use self::term_painter::{ToStyle};
use self::term_painter::Color::*;


pub fn do_remove(mut number: usize,base: &str) -> Result<(),error::BookError> {
  if number >= 1 {
    number = number - 1;
  }

  let mut dir_entries = try!(util::sorted_dir_entries(base));

  if number >= dir_entries.len() {
    number = dir_entries.len()-1;
  }
  let dir_name = dir_entries[number].to_string();
  remove_dir!(base,dir_name);
  let highest_value = dir_entries.len()-1;
  if highest_value > number {
    try!(util::move_section_dirs(number+1,highest_value,base,&dir_entries));
    dir_entries.remove(number);
    let highest_value = dir_entries.len()-1;
    util::rearrange_entries(number,highest_value,&mut dir_entries);
  } else {
    dir_entries.remove(number);
  }
  try!(util::rewrite_index(&mut dir_entries,base));
  Ok(())
}
