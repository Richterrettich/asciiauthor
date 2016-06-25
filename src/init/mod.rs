extern crate clap;
extern crate term_painter;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Write};
use util;
use error;



pub fn init (name: &str,  base: &str) -> Result<(),error::BookError> {
  let dir_name = util::replace_spaces(name);
  let dir_path = format!("{}/{}",base,dir_name);
  create_dir!(&dir_path,"content");
  create_dir!(&dir_path,"includes");
  create_dir!(&dir_path,"content/images");

  create_file!(&dir_path,".gitignore",
  "**/*.html\n\
  **/*.pdf\n\
  **/*.pdfmarks\n\
  **/*.textclippings\n\
  **/.DS_Store\n");
  create_file!(&dir_path,"includes/config.adoc",
  ":icons: font\n\
  :toc: macro\n\
  :stem: latexmath\n\
  :source-highlighter: coderay\n\
  :listing-caption: Listing\n\
  :pdf-page-size: A4\n");
  create_file!(&dir_path,"content/index.adoc",
  "= {}\n\
  include::../includes/config.adoc[]\n\n\
  :content: .\n\n\
  toc::[]\n\n",name);
  println!("All done!");
  Ok(())
}
