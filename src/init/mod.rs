extern crate clap;
extern crate git2;
extern crate term_painter;

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;
use self::git2::{Repository,Config};
use std::fs;
use std::fs::{File};
use std::path::Path;
use std::io::{Write,Error};



pub fn init (name: &str, user_email: &str,user_name: &str) -> Result<(),Error> {
  create_dir!(name,"content");
  create_dir!(name,"includes");
  create_dir!(name,"content/images");
  println!("{}  git repository",Green.bold().paint("Initialize"));
  Repository::init(name).ok();
  println!("{}  git config",Green.bold().paint("Append"));

  let mut cfg = Config::open(Path::new(&*format!("{}/.git/config",name)))
                            .ok()
                            .expect("couldn't open config");

  cfg.set_str("push.default","simple").ok();
  create_file!(name,".gitignore",
  "**/*.html\n\
  **/*.pdf\n\
  **/*.pdfmarks\n\
  **/*.textclippings\n\
  **/.DS_Store\n");
  create_file!(name,"includes/config.adoc",
  ":icons: font\n\
  :toc: macro\n\
  :stem: latexmath\n\
  :source-highlighter: coderay\n\
  :listing-caption: Listing\n\
  :pdf-page-size: A4\n");
  create_file!(name,"content/index.adoc",
  "= {}\n\
  {} <{}>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n",name.split('/').last().unwrap(),user_name,user_email);
  create_file!(name,".git/description","{}_book",name);
  println!("All done!");
  Ok(())
}
