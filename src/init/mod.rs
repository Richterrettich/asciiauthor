extern crate clap;
extern crate git2;
extern crate term_painter;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::git2::{Repository,Config};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Write};
use util;
use error;



pub fn init (name: &str, user_email: &str,user_name: &str,base: &str) -> Result<(),error::BookError> {
  let dir_name = util::replace_spaces(name);
  let dir_path = format!("{}/{}",base,dir_name);
  create_dir!(&dir_path,"content");
  create_dir!(&dir_path,"includes");
  create_dir!(&dir_path,"content/images");
  println!("{}  git repository",Green.bold().paint("Initialize"));
  try!(Repository::init(&dir_path));

  println!("{}  git config",Green.bold().paint("Append"));

  let mut cfg = Config::open(Path::new(&*format!("{}/.git/config",&dir_path)))
                            .ok()
                            .expect("couldn't open config");

  cfg.set_str("push.default","simple").ok();
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
  {} <{}>\n\
  include::../includes/config.adoc[]\n\n\
  :content: .\n\n\
  toc::[]\n\n",name,user_name,user_email);
  create_file!(&dir_path,".git/description","{}_book",dir_name);
  println!("All done!");
  Ok(())
}
