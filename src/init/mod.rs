extern crate clap;
extern crate git2;
extern crate term_painter;

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;
use self::git2::{Repository,Config};
use std::env;
use std::fs;
use std::fs::{File};
use std::path::Path;
use std::io::{Write,Error};
use std::io;
use self::git2::ConfigLevel;



pub fn init (name: &str) -> Result<(),Error> {

  let user_name = get_user_information("user.name");
  let user_email = get_user_information("user.email");
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
  :imagesdir: ./images\n\
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

fn get_user_information(property : &str) -> String {
  let mut cfg = match Config::open_default() {
    Ok(c) => c,
    Err(_err) => create_git_config(env::home_dir().unwrap().as_path()),
  };
  let snapshot = cfg.snapshot().ok().expect("can't create a config-snapshot!");
  match snapshot.get_str(property) {
     Ok(name) => name.to_string(),
     Err(_err) => request_user_information(&mut cfg,property),
  }
}

fn create_git_config(path : &Path) -> Config {
  let mut config = Config::new().unwrap();
  config.add_file(path, ConfigLevel::Global, false).ok();
  config
}

fn request_user_information(config : &mut Config, property: &str) -> String {
  let mut property_value = String::new();
  let prompt_value = property.split('.').last().expect("There should be some!");
  print!("it seems that you don't have set your {} yet. Please enter your {}\n> ",prompt_value,prompt_value);
  io::stdout().flush().ok();
  io::stdin().read_line(&mut property_value)
      .ok()
      .expect("Failed to read line");
  println!("");
  property_value = property_value.trim_matches('\n').to_string();
  config.set_str(property,&*property_value).ok();
  property_value
}
