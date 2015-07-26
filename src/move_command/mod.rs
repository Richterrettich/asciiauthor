use swap_command;
use error;
use std::collections::BTreeMap;
use std::fs;
use std::io::{Read,Write,Error};



pub fn do_move(first: u16, second: u16, dir: &str) -> Result<(),error::BookError> {

  let dir_entries = try!(swap_command::sorted_dir_entries(dir));

  if dir_entries.contains_key(&first) {

    Ok(())
  } else {
    Err(error::BookError::NormalBookError(format!("can't move section {} to {}",first,second)))
  }

}

fn move_dirs(start: u16, end: u16,base: &str,dir_entries: &BTreeMap<u16,String>) -> Result<(),error::BookError> {
  try!(fs::rename(dir(base,start,dir_entries),
                  assemble_dir_name(base,end,dir_entries.get(&start).unwrap())));
  if start > end {
    for_each_dir(start-1,end,base,dir_entries,|entry,index| {
      fs::rename(entry,assemble_dir_name(base,index+1,dir_entries.get(&index).unwrap())).unwrap();
    });
  } else {
    for_each_dir(start+1,end,base,dir_entries,|entry,index| {
      fs::rename(entry,assemble_dir_name(base,index-1,dir_entries.get(&index).unwrap())).unwrap();
    });
  }
  Ok(())
}

fn move_index_entries(start: u16, end: u16,base: &str,dir_entries: &BTreeMap<u16,String>) -> Result<(),error::BookError> {
  let mut f = try!(fs::File::open(format!("{}/index.adoc",base)));
  let mut file_content = String::new();
  try!(f.read_to_string(&mut file_content));

  if start > end {
    let start_dir = dir(base,end,dir_entries);
    let end_dir = dir(base,start,dir_entries);
    let mut parts = file_content.split(&*start_dir);
    let mut first_part = parts.next().unwrap().to_string();
    let mut new_content = String::new();
    let mut second_parts: Vec<&str>  = parts.next().unwrap().split(&*end_dir).collect();
    for i in start-1 .. end {
      let name  = format!("{}_{}",i+1,dir_entries.get(&i).unwrap());
      new_content.push_str(&*format!("include::{}/index.adoc[]\n\n",name));
    }
    second_parts[0] = &*new_content;
    first_part.push_str(&*second_parts.connect(""));
    let mut temp_file = try!(fs::File::create(format!("{}/temp_file",&base)));
    try!(write!(temp_file,"{}",first_part));
    try!(fs::rename(format!("{}/temp_file",&base),format!("{}/index.adoc",&base)));
  } else {
    let start_dir = dir(base,start,dir_entries);
    let end_dir = dir(base,end,dir_entries);
    let mut parts = file_content.split(&*start_dir);
    let mut first_part = parts.next().unwrap().to_string();
    let mut new_content = String::new();
    let mut second_parts: Vec<&str> = parts.next().unwrap().split(&*end_dir).collect();
    for i in start+1 .. end {
      let name  = format!("{}_{}",i-1,dir_entries.get(&i).unwrap());
      new_content.push_str(&*format!("include::{}/index.adoc[]\n\n",name));
    }
    second_parts[0] = &*new_content;
     first_part.push_str(&*second_parts.connect(""));
    let mut temp_file = try!(fs::File::create(format!("{}/temp_file",&base)));
    try!(write!(temp_file,"{}",first_part));
    try!(fs::rename(format!("{}/temp_file",&base),format!("{}/index.adoc",&base)));
  }

  Ok(())


}

fn for_each_dir<F>(start: u16, end: u16,base: &str,dir_entries: &BTreeMap<u16,String>,f: F)  where F: Fn(&str,u16) {
  for i in start .. end {
    f(&*dir(base,i,dir_entries),i);
  }
}

fn dir(base: &str, number: u16, dir_entries: &BTreeMap<u16,String>) -> String {
  let name  = dir_entries.get(&number).unwrap();
  assemble_dir_name(base,number,name)
}

fn assemble_dir_name(base: &str, number: u16,name: &str) -> String {
  format!("{}/{}_{}",base,number,name)
}
