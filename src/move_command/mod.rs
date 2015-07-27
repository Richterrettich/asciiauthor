extern crate term_painter;

use swap_command;
use error;
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read,Write};

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;



pub fn do_move(mut first: u16, mut second: u16, base: &str) -> Result<(),error::BookError> {



  let dir_entries = try!(swap_command::sorted_dir_entries(base));

  let highest_value = get_last_section_number(&dir_entries);

  if second > highest_value {
    second = highest_value;
  }
  if first < 1 {
    first = 1;
  }

  println!("{}  {}_{} to {}_{}",Blue.bold().paint("Move"),first,dir_entries.get(&first).unwrap(),second,dir_entries.get(&second).unwrap());

  if dir_entries.contains_key(&first) && dir_entries.contains_key(&second) {
    try!(fs::rename(dir(base,first,&dir_entries),
                    assemble_dir_name(base,second,dir_entries.get(&first).unwrap())));
    try!(move_section_dirs(first,second,base,&dir_entries));
    try!(move_index_entries(first,second,base,&dir_entries));
    Ok(())
  } else {
    Err(error::BookError::NormalBookError(format!("can't move section {} to {}",first,second)))
  }
}


fn move_index_entries(start: u16, end: u16,base: &str,dir_entries: &BTreeMap<u16,String>) -> Result<(),error::BookError> {
  let mut f = try!(fs::File::open(format!("{}/index.adoc",base)));
  let mut file_content = String::new();
  try!(f.read_to_string(&mut file_content));

  if start > end {
    try!(reallocate_index(start,end,base,dir_entries,|first_part,new_content,last_part|{
      format!("{}include::{}_{}/index.adoc[]\n\n{}{}",first_part,end,dir_entries.get(&start).unwrap(),new_content,last_part)
    }));

  } else {
    try!(reallocate_index(start,end,base,dir_entries,|first_part,new_content,last_part|{
      format!("{}{}include::{}_{}/index.adoc[]\n\n{}",first_part,new_content,end,dir_entries.get(&start).unwrap(),last_part)
    }));
  }

  Ok(())
}

pub fn get_last_section_number(dir_entries: &BTreeMap<u16,String>) -> u16 {
  let keys: Vec<&u16> = dir_entries.keys().collect();
  let mut highest_value = 0;
  for &num in keys {
    if num > highest_value {
      highest_value = num;
    }
  }
  highest_value
}

pub fn reallocate_index<F>(start: u16, end:u16,base: &str,dir_entries: &BTreeMap<u16,String>, func: F)
                                      -> Result<(),error::BookError>  where F: Fn(&str,&str,&str) -> String {
  let mut f = try!(fs::File::open(format!("{}/index.adoc",base)));
  let mut file_content = String::new();
  try!(f.read_to_string(&mut file_content));

  let mut new_content = String::new();
  if start > end {
    let start_split_point = format!("include::{}_{}/index.adoc[]\n\n",end,dir_entries.get(&end).unwrap());
    let end_split_point = format!("include::{}_{}/index.adoc[]\n\n",start,dir_entries.get(&start).unwrap());
    let mut parts = file_content.split(&*start_split_point);
    let first_part = parts.next().unwrap().to_string();
    let second_parts: Vec<&str>  = parts.next().expect("problem with your second split").split(&*end_split_point).collect();
    for i in end .. start {
      let name  = format!("{}_{}",i+1,dir_entries.get(&i).expect("problem within your loop"));
      new_content.push_str(&*format!("include::{}/index.adoc[]\n\n",name));
    }
    let result = if second_parts.len() > 1 {
      func(&*first_part,&*new_content,second_parts[1])
    } else {
      func(&*first_part,&*new_content,"")
    };
    let mut temp_file = try!(fs::File::create(format!("{}/temp_file",&base)));
    try!(write!(temp_file,"{}",result));
    try!(fs::rename(format!("{}/temp_file",&base),format!("{}/index.adoc",&base)));
  }
  else {
    let start_split_point = format!("include::{}_{}/index.adoc[]\n\n",start,dir_entries.get(&start).unwrap());
    let end_split_point = format!("include::{}_{}/index.adoc[]\n\n",end,dir_entries.get(&end).unwrap());
    let mut parts = file_content.split(&*start_split_point);
    let first_part = parts.next().unwrap().to_string();
    let second_parts: Vec<&str>  = parts.next().expect("problem with your second split").split(&*end_split_point).collect();
    for i in start+1 .. end+1 {
      let name  = format!("{}_{}",i-1,dir_entries.get(&i).expect("problem within your loop"));
      new_content.push_str(&*format!("include::{}/index.adoc[]\n\n",name));
    }
    let result = if second_parts.len() > 1 {
      func(&*first_part,&*new_content,second_parts[1])
    } else {
      func(&*first_part,&*new_content,"")
    };
    let mut temp_file = try!(fs::File::create(format!("{}/temp_file",&base)));
    try!(write!(temp_file,"{}",result));
    try!(fs::rename(format!("{}/temp_file",&base),format!("{}/index.adoc",&base)));
  }
  Ok(())
}

pub fn move_section_dirs(start: u16, end:u16,base: &str,dir_entries: &BTreeMap<u16,String>) -> Result<(),error::BookError> {
  if start > end {
    for i in end .. start {
      try!(fs::rename(dir(base,i,&dir_entries),assemble_dir_name(base,i+1,dir_entries.get(&i).unwrap())));
    }
  } else {
    for i in start+1 .. end+1 {
      try!(fs::rename(dir(base,i,&dir_entries),assemble_dir_name(base,i-1,dir_entries.get(&i).unwrap())));
    }
  }
  Ok(())
}

fn dir(base: &str, number: u16, dir_entries: &BTreeMap<u16,String>) -> String {
  let name  = dir_entries.get(&number).unwrap();
  assemble_dir_name(base,number,name)
}

fn assemble_dir_name(base: &str, number: u16,name: &str) -> String {
  format!("{}/{}_{}",base,number,name)
}
