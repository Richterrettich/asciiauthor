use std::fs;
use std::num;
use error;
use std::io::{Read,Write,Error};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;



pub struct Section {
  pub  position:  usize,
  pub  name:  String
}

impl Section {
  fn new(position:usize,name:String) -> Section {
    Section{position:position,name:name}
  }

  pub fn to_string(&self) -> String {
    format!("{}_{}",self.position,self.name)
  }
}

impl PartialEq for Section {
  fn eq(&self, other: &Self) -> bool {
    self.position == other.position
  }
}

impl Eq for Section {

}

impl PartialOrd for Section {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.position > other.position {
      return Some(Ordering::Greater);
    } else if self.position < other.position {
      return Some(Ordering::Less);
    }
    Some(Ordering::Equal)
  }
}

impl Ord for Section {
  fn cmp(&self, other: &Self) -> Ordering {
    if self.position > other.position {
      return Ordering::Greater;
    } else if self.position < other.position {
      return Ordering::Less;
    }
    Ordering::Equal
  }
}



pub fn rewrite_index(dir_entries: &mut Vec<Section>,base: &str) -> Result<(),error::BookError> {
  dir_entries.sort();
  let mut f = try!(fs::File::open(format!("{}/index.adoc",base)));
  let mut file_content = String::new();
  try!(f.read_to_string(&mut file_content));
  let first_part = if file_content.contains("//BEGIN SECTIONS") {
    file_content.split("//BEGIN SECTIONS\n").next().unwrap().to_string()
  } else {
    file_content
  };
  let mut includes_part = "//BEGIN SECTIONS\n".to_string();
  for entry in dir_entries {
    let entry_name = entry.to_string();
//    let image_include = get_image_path(base,&*entry_name);
    includes_part.push_str(&*format!(
      "include::{}/index.adoc[]\n\n",entry_name));
  }
  let mut temp_file = try!(fs::File::create(format!("{}/temp_file",base)));
  try!(write!(temp_file,"{}{}",first_part,includes_part));
  try!(fs::rename(format!("{}/temp_file",&base),format!("{}/index.adoc",&base)));
  Ok(())
}

pub fn rewrite_sections(dir_entries: &mut Vec<Section>,base: &str) -> Result<(),error::BookError> {
  for dir_entry in dir_entries {
    let index_name = format!("{}/{}/index.adoc",base,dir_entry.to_string());
    let file = try!(fs::File::open(index_name));
    let temp_file = try!(fs::File::create(format!("{}/{}/temp_file",base,dir_entry.to_string())));
    let reader = BufReader::new(&file);
    for line in reader.lines() {
      let line_string = line.unwrap();
      let append_line = if line_string.starts_with(&*format!(":{}: {{",dir_entry.name)) {
        format!(":{}: {{{}}}/{}",dir_entry.name,extract_parent_variable(base),dir_entry.to_string())
      } else {
        line_string
      };
      try!(write!(&temp_file,"{}\n",append_line));
    }
    try!(fs::rename(format!("{}/{}/temp_file",base,dir_entry.to_string()),format!("{}/{}/index.adoc",base,dir_entry.to_string())));
  }
  Ok(())
}

pub fn extract_parent_variable(path: &str) -> String {
  let path_object = Path::new(path);
  let mut parent_image_variable = path_object.file_name().unwrap().to_str().unwrap().to_string();
  if parent_image_variable != "content" {
    parent_image_variable = parent_image_variable.split('_').skip(1).collect::<Vec<&str>>().join("_");
  }
  parent_image_variable
}


pub fn move_section_dirs(start: usize, end:usize,base: &str,dir_entries: &Vec<Section>) -> Result<(),error::BookError> {
  if start > end {
    for i in end .. start+1 {
      try!(fs::rename(dir(base,i,&dir_entries),
                      assemble_dir_name(base,dir_entries[i].position+1,&*dir_entries[i].name)));
    }
  } else {
    for i in start .. end+1 {
      try!(fs::rename(dir(base,i,&dir_entries),
                      assemble_dir_name(base,dir_entries[i].position-1,&*dir_entries[i].name)));
    }
  }
  Ok(())
}


pub fn dir(base: &str, index: usize, dir_entries: &Vec<Section>) -> String {
  let name  = &dir_entries[index].name;
  let number =  &dir_entries[index].position;
  assemble_dir_name(base,*number,name)
}

pub fn assemble_dir_name(base: &str, number: usize,name: &str) -> String {
  format!("{}/{}_{}",base,number,name)
}

pub fn sorted_dir_entries(path: &str) -> Result<Vec<Section>,Error> {
  let read_dir = try!(fs::read_dir(path));
  let result: Vec<Section> = read_dir.filter_map(|item| item.ok())
          .filter(|entry| entry.file_type().unwrap().is_dir())
          .map(|dir| dir.file_name())
          .map(|file_name| file_name.to_str().unwrap().to_string())
          .filter(|dir_name| dir_name.contains("_"))
          .filter_map(|dir_name| extract_number_value(&*dir_name).ok())
          .map(|(number,name)| Section::new(number,name))
          .collect();
  Ok(result)
}

pub fn extract_number_value(value:&str) -> Result<(usize,String),num::ParseIntError> {
  let mut iter = value.split('_');
  match iter.next().unwrap().parse::<usize>() {
    Ok(number) => Ok((number,iter.collect::<Vec<&str>>().join("_"))),

    Err(err) => Err(err)
  }
}

pub fn split_name(raw_name: &str) -> (&str,&str) {
  let path = Path::new(raw_name);
  match path.parent() {
    Some(parent) => {
      let mut parent_str = parent.to_str().unwrap();
      parent_str = if parent_str == "" {
        "."
      } else {
        parent_str
      };
      (path.file_name().unwrap().to_str().unwrap(),parent_str)
    },
    None => (path.file_name().unwrap().to_str().unwrap(),"/"),
  }
}

pub fn rearrange_entries(first: usize, last:usize,dir_entries: &mut Vec<Section>) {
  if first > last {
    for i in last .. first+1 {
      dir_entries[i].position = dir_entries[i].position+1;
    }
  } else {
    for i in first .. last+1 {
      dir_entries[i].position = dir_entries[i].position-1;
    }
  }
}


pub fn replace_spaces(title: &str) -> String {
  if title.contains(' ') {
    title.split(' ').collect::<Vec<&str>>().join("_")
  } else {
    title.to_string()
  }
}

pub fn reset_project() {

    /*println!("syncing working directory...");
    match repo.reset(&commit_object,ResetType::Hard,None) {
        Ok(_empty) => Ok(()),
        Err(err) => Err(format!("something went wrong while syncing: {}",err))
    }*/
}


pub fn get_image_path(path: &str, dir_name: &str) -> String {
  let path_parts: Vec<&str> = path.split("/content/").collect();
  if path_parts.len() > 1 {
    format!("{}/{}",path_parts.last().unwrap(),dir_name)
  } else {
    dir_name.to_string()
  }
}
