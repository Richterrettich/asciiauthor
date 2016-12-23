extern crate clap;
extern crate term_painter;
extern crate argf;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Write, Error, BufReader, BufRead};
use error;
use util;
use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use std::fs::OpenOptions;



enum Location {
    InScope(String, usize),
    OutOfScope,
}

pub fn section(name: &str, dir: &str, inputs: Option<Vec<&str>>) -> Result<(), error::BookError> {
    match find_content_root(dir) {
        Location::InScope(_path, level) => add_part(name, dir, level, inputs),
        Location::OutOfScope => {
            Err(error::BookError::NormalBookError("not within project directory.".to_string()))
        }
    }
}

fn add_part(title: &str,
            path: &str,
            level: usize,
            inputs: Option<Vec<&str>>)
            -> Result<(), error::BookError> {

    let dir_name = util::replace_spaces(title);

    let new_number = try!(find_last_number(path)) + 1;
    create_dir!(path, &*format!("{}_{}", new_number, dir_name));

    let image_include = util::get_image_path(path, &*format!("{}_{}", new_number, dir_name));
    println!("path: {}", path);
    println!("image_include: {}", image_include);
    create_dir!(path, &*format!("{}_{}/images", new_number, dir_name));
    let mut headings = "=".to_string();
    let mut options_include = "include::../".to_string();
    for _i in 0..level {
        headings.push_str("=");
        options_include.push_str("../")
    }

    let parent_image_variable = util::extract_parent_variable(path);
    let section_name = format!("{}_{}", new_number, dir_name);
    options_include.push_str("includes/config.adoc[]\n");
    create_file!(path,
                 &*format!("{}/index.adoc", section_name),
                 "{}\n{} {}\n:{}: .\nifdef::{}[]\n:{}: {{{}}}/{}\nendif::{}[]\n:imagesdir: \
                  {{{}}}/images\n\n",
                 options_include,
                 headings,
                 title,
                 dir_name,
                 parent_image_variable,
                 dir_name,
                 parent_image_variable,
                 section_name,
                 parent_image_variable,
                 dir_name);

    if new_number == 1 {
        append_file!(&*format!("{}/index.adoc", path),
                     "//BEGIN SECTIONS\ninclude::{}/index.adoc[]\n\n",
                     &*section_name);
    } else {
        append_file!(&*format!("{}/index.adoc", path),
                     "include::{}/index.adoc[]\n\n",
                     &*section_name);
    }

    if let Some(input_files) = inputs {
        let f = BufReader::new(argf::from_iterator(input_files.iter()
            .map(|x| x.to_string())));
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&*format!("{}/index.adoc", path))
            .unwrap();
        for line in f.lines().filter_map(|result| result.ok()) {
            file.write_all(line.as_bytes()).expect("Unable to write data");
        }
    }
    Ok(())
}


fn find_last_number(path: &str) -> Result<usize, Error> {
    let mut highest_number = 0;
    for entry in try!(fs::read_dir(path)) {
        let dir = try!(entry);
        if dir.file_type().unwrap().is_dir() {
            let raw_file_name = dir.file_name();
            let file_name = raw_file_name.to_str().unwrap();
            if file_name.contains("_") {
                let first_parts: Vec<&str> = file_name.split("_").collect();
                if let Ok(number) = first_parts[0].parse::<usize>() {
                    highest_number = if number > highest_number {
                        number
                    } else {
                        highest_number
                    };
                }
            }
        }
    }
    Ok(highest_number)
}


fn find_content_root(p: &str) -> Location {
    let file_name = Path::new(p).file_name().unwrap().to_str().unwrap();
    let (possible_root, depth) = if file_name == "content" {
        (p.to_string(), 1)
    } else {
        let parts: Vec<&str> = p.split("/content/").collect();
        if parts.len() >= 1 {
            let last_bits: Vec<&str> = parts.last().unwrap().split("/").collect();
            (format!("{}/content", parts[0]), last_bits.len() + 1)
        } else {
            return Location::OutOfScope;
        }
    };

    // TODO find more robust way to figure out project root.
    return Location::InScope(possible_root.to_string(), depth);
}
