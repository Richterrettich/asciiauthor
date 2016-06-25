extern crate term_painter;

use util;
use error;
use std::fs;

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;



pub fn do_move(mut first: usize, mut second: usize, base: &str) -> Result<(),error::BookError> {

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
  println!("{}  {} to {}",Blue.bold().paint("Move"),
                         dir_entries[first].to_string(),
                         dir_entries[second].to_string());

  try!(fs::rename(util::dir(base,first,&dir_entries),
                  util::assemble_dir_name(base,dir_entries[second].position,&*dir_entries[first].name)));
  let target_value = dir_entries[second].position;
  if first > second {
    try!(util::move_section_dirs(first-1,second,base,&dir_entries));
    util::rearrange_entries(first,second,&mut dir_entries);
  } else {
    try!(util::move_section_dirs(first+1,second,base,&dir_entries));
    util::rearrange_entries(first,second,&mut dir_entries);
  }

  dir_entries[first].position = target_value;
  try!(util::rewrite_index(&mut dir_entries,base));
  try!(util::rewrite_sections(&mut dir_entries,base));

  Ok(())
}
