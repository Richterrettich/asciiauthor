
#[macro_export]
macro_rules! create_file {
  ($b:expr,$p:expr,$c:expr) => {{
    let path = &*format!("{}/{}",$b,$p);
    println!("{}  {}",Green.bold().paint("Create File"),path);
    let f = try!(File::create(&*format!("{}",path)));
    try!(write!(&f,$c));
  }};
  ($b:expr,$p:expr,$c:expr,$( $sub:expr ),*) => {{
    let path = &*format!("{}/{}",$b,$p);
    println!("{}  {}",Green.bold().paint("Create File"),path);
    let f = try!(File::create(&*format!("{}",path)));
    try!(write!(&f,$c,$($sub),*));
    }};
}

#[macro_export]
macro_rules! create_dir {
  ($b:expr,$p:expr) => {{
    let path = &*format!("{}/{}",$b,$p);
    println!("{}  {}",Green.bold().paint("Create Directory"),path);
    try!(fs::create_dir_all(Path::new(path)));
  }}
}

#[macro_export]
macro_rules! check_location {
  ($b:expr,$p:expr) => {{
    let path = &*format!("{}/{}",$b,$p);
    println!("{}  {}",Green.bold().paint("Create Directory"),path);
    try!(fs::create_dir_all(Path::new(path)));
  }}
}

pub mod init; // exports the module defined in init/mod.rs
pub mod chapter;
pub mod section;
pub mod error;
