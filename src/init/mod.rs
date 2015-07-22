extern crate clap;
extern crate git2;
extern crate term_painter;

use self::term_painter::{ToStyle};
use self::term_painter::Color::*;
use self::clap::ArgMatches;
use self::git2::{Repository,Config};
use std::env;
use std::fs;
use std::fs::{File};
use std::path::Path;
use std::io::{Write,Error};
use std::io;
use self::git2::ConfigLevel;



pub fn init (arguments: &ArgMatches) -> Result<(),Error> {

  let name = arguments.value_of("name").unwrap();//safe. name is required argument.
  let user_name = get_user_information("user.name");
  let user_email = get_user_information("user.email");
  create_dir!(name,"src");
  create_dir!(name,"includes");
  println!("{}  git repository",Green.bold().paint("Initialize"));
  Repository::init(name).ok();
  println!("{}  git config",Green.bold().paint("Append"));

  let mut cfg = Config::open(Path::new(&*format!("{}/.git/config",name)))
                            .ok()
                            .expect("couldn't open config");

  cfg.set_str("push.default","simple").ok();
  create_file!(name,".gitignore",
"**/*.html
**/*.pdf
**/*.pdfmarks
**/*.textclippings
**/.DS_Store
");
  create_file!(name,"includes/config.adoc",
":icons: font
:toc: macro
:stem: latexmath
:source-highlighter: coderay
:listing-caption: Listing
:pdf-page-size: A4
");
  create_file!(name,"src/index.adoc",
"= {}
{} <{}>
include::../includes/config.adoc[]

toc::[]
",name,user_name,user_email);

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
