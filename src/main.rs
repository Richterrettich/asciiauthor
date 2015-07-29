#[macro_use]
extern crate clap;
extern crate asciiauthor;
extern crate git2;
use clap::{Arg, App, SubCommand};
use asciiauthor::*;
use std::env;
use git2::{Config,ConfigLevel};
use std::path::Path;
use std::io::{Write,Error};
use std::io;


fn main() {
  let matches = App::new("asciiauthor")
                        .version("0.1.0")
                        .author("Rene Richter <Richterrettich@gmail.com>")
                        .about("Project tool for asciidoctor")
                        .subcommand_required_else_help(true)
                        .versionless_subcommands(true)
                        .subcommand(SubCommand::with_name("init")
                                    .about("initializes a new book project")
                                    .arg(Arg::with_name("name")
                                        .required(true)
                                        .help("The name of the book")))
                        .subcommand(SubCommand::with_name("section")
                                    .args_from_usage("<NAMES>... 'names of the sections.'"))
                                    //.about("creates a new section"))
                        .subcommand(SubCommand::with_name("swap")
                                    .about("swaps a section")
                                    .arg(Arg::with_name("old_number")
                                          .required(true)
                                          .help("the first section to swap."))
                                    .arg(Arg::with_name("new_number")
                                          .required(true)
                                          .help("the second section to swap.")))
                        .subcommand(SubCommand::with_name("move")
                                    .about("moves a section")
                                    .arg(Arg::with_name("old_number")
                                          .required(true)
                                          .help("the old number of the section."))
                                    .arg(Arg::with_name("new_number")
                                          .required(true)
                                          .help("the new number of the section.")))
                        .subcommand(SubCommand::with_name("delete")
                                    .about("deletes a section")
                                    .arg(Arg::with_name("number")
                                          .required(true)
                                          .help("the number of the section to delete.")))
                        .get_matches();

  let curret_dir = env::current_dir().unwrap();
  let p = curret_dir.to_str().unwrap();
  match matches.subcommand() {
      ("init", Some(matches))   => {
        let user_name = get_user_information("user.name");
        let user_email = get_user_information("user.email");
        let raw_name = matches.value_of("name").unwrap();
        let (name,base) = util::split_name(raw_name);
        print_result(init::init(name,&*user_email,&*user_name,base))
      },
      ("section", Some(matches)) => {
          for v in matches.values_of("NAMES").unwrap() {
         		print_result(section::section(v,p));
        	}
      },
      ("swap", Some(matches)) => {
        let old = value_t_or_exit!(matches.value_of("old_number"),usize);
        let new = value_t_or_exit!(matches.value_of("new_number"),usize);
        print_result(swap_command::do_swap(old,new,p))
      },
      ("move", Some(matches)) => {
        let old = value_t_or_exit!(matches.value_of("old_number"),usize);
        let new = value_t_or_exit!(matches.value_of("new_number"),usize);
        print_result(move_command::do_move(old,new,p))
      },
      ("delete", Some(matches)) => {
        let number = value_t_or_exit!(matches.value_of("number"),usize);
        print_result(delete_command::do_remove(number,p))
      },

      _                         => {},
  };
}


fn print_result<T: std::error::Error>(result: Result<(),T>) {
  match result {
    Ok(()) => {},
    Err(err) => println!("{}",err.description())
  }
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
