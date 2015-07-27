#[macro_use]
extern crate clap;
extern crate ghostwriter;
use clap::{Arg, App, SubCommand};
use ghostwriter::*;
use std::env;


fn main() {
  let matches = App::new("myapp")
                        .version("1.0")
                        .author("Rene Richter <Richterrettich@gmail.com>")
                        .about("Does awesome things")
                        .subcommand_required_else_help(true)
                        .versionless_subcommands(true)
                        .subcommand(SubCommand::with_name("init")
                                    .about("initializes a new book project")
                                    .arg(Arg::with_name("name")
                                        .required(true)
                                        .help("The name of the book")))
                        .subcommand(SubCommand::with_name("section")
                                    .about("creates a new section")
                                    .arg(Arg::with_name("name")
                                          .required(true)
                                          .help("the name of the section.")))
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
      ("init", Some(matches))   => print_result(init::init(matches.value_of("name").unwrap())),
      ("section", Some(matches)) => {
        print_result(section::section(matches.value_of("name").unwrap(),p))
      },
      ("swap", Some(matches)) => {
        let old = value_t_or_exit!(matches.value_of("old_number"),u16);
        let new = value_t_or_exit!(matches.value_of("new_number"),u16);
        print_result(swap_command::do_swap(old,new,p))
      },
      ("move", Some(matches)) => {
        let old = value_t_or_exit!(matches.value_of("old_number"),u16);
        let new = value_t_or_exit!(matches.value_of("new_number"),u16);
        print_result(move_command::do_move(old,new,p))
      },
      ("delete", Some(matches)) => {
        let number = value_t_or_exit!(matches.value_of("number"),u16);
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
