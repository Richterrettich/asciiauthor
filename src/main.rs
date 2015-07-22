extern crate clap;
extern crate ghostwriter;
use clap::{Arg, App, SubCommand};
use ghostwriter::*;

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
                        .subcommand(SubCommand::with_name("chapter")
                                    .about("creates a new chapter")
                                    .arg(Arg::with_name("name")
                                         .required(true)
                                         .help("the name of the chapter.")))
                        .subcommand(SubCommand::with_name("section")
                                    .about("creates a new section")
                                    .arg(Arg::with_name("name")
                                          .required(true)
                                          .help("the name of the section.")))
                        .get_matches();

  let result = match matches.subcommand() {
      ("init", Some(matches))   => init::init(matches),
      ("chapter", Some(matches)) => chapter::chapter(&matches),
      ("section", Some(matches)) => section::section(&matches),
      _                         => Ok(()),
  };

  match result {
    Err(err) => println!("{}",err),
    _ => {}
  };


}
