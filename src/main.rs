#[macro_use]
extern crate clap;
extern crate asciiauthor;
use clap::{Arg, App, SubCommand, AppSettings};
use asciiauthor::*;
use std::env;

fn main() {
    let matches = App::new("asciiauthor")
        .version("0.1.0")
        .author("Rene Richter <Richterrettich@gmail.com>")
        .about("Project tool for asciidoctor")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("init")
            .about("initializes a new book project")
            .arg(Arg::with_name("name")
                .required(true)
                .help("The name of the book")))
        .subcommand(SubCommand::with_name("sections")
            .args_from_usage("<NAMES>... 'names of the sections.'")
            .about("creates a set of new sections"))
        .subcommand(SubCommand::with_name("section")
            .arg(Arg::with_name("NAME")
                .required(true)
                .help("The name of the section")
                .index(1))
            .arg(Arg::with_name("INPUTS")
                .required(false)
                .help("A possible input for importing content")
                .index(2)
                .multiple(true))
            .about("creates a single new section with an optional template"))
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
        .subcommand(SubCommand::with_name("compile")
            .about("compiles the project into html")
            .arg(Arg::with_name("backend")
                .short("b")
                .long("backend")
                .value_name("FILE")
                .help("the backend to use. Defaults to 'asciidoctor'")
                .takes_value(true)))
        .get_matches();

    let curret_dir = env::current_dir().unwrap();
    let p = curret_dir.to_str().unwrap();
    match matches.subcommand() {
        ("init", Some(matches)) => {
            let raw_name = matches.value_of("name").unwrap();
            let (name, base) = util::split_name(raw_name);
            print_result(init::init(name, base))
        }
        ("section", Some(matches)) => {
            let name = matches.value_of("NAME").unwrap();
            let inputs = matches.values_of("INPUTS");
            print_result(section::section(name, p, inputs));
        }
        ("sections", Some(matches)) => {
            for v in matches.values_of("NAMES").unwrap() {
                print_result(section::section(v, p, None));
            }
        }
        ("swap", Some(matches)) => {
            let old = value_t_or_exit!(matches.value_of("old_number"), usize);
            let new = value_t_or_exit!(matches.value_of("new_number"), usize);
            print_result(swap_command::do_swap(old, new, p))
        }
        ("move", Some(matches)) => {
            let old = value_t_or_exit!(matches.value_of("old_number"), usize);
            let new = value_t_or_exit!(matches.value_of("new_number"), usize);
            print_result(move_command::do_move(old, new, p))
        }
        ("delete", Some(matches)) => {
            let number = value_t_or_exit!(matches.value_of("number"), usize);
            print_result(delete_command::do_remove(number, p))
        }
        ("compile", Some(matches)) => {
            if let Some(backend) = matches.value_of("backend") {
                println!("{}",backend);
                print_result(compile_command::compile(p,backend));
            } else {
                print_result(compile_command::compile(p,"asciidoctor"));
            }
        }

        _ => {}
    };
}


fn print_result<T: std::error::Error>(result: Result<(), T>) {
    match result {
        Ok(()) => {}
        Err(err) => println!("{}", err.description()),
    }
}
