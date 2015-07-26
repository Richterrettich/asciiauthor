extern crate ghostwriter;
use ghostwriter::*;
use std::fs;
use std::io::Read;

const  TEST_PROJECT_ROOT: &'static str = "test";

#[test]
fn it_should_create_a_valid_project() {
  let test_project = format!("{}/init_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let init_result = init::init(&*test_project);

  assert!(init_result.is_ok());

  let entries: Vec<fs::DirEntry> = fs::read_dir(&test_project).unwrap()
                                                .filter_map(|item| item.ok())
                                                .collect();
  assert_eq!(4,entries.len());

  assert_file_content(&*format!("{}/.gitignore",&test_project),"**/*.html\n\
  **/*.pdf\n\
  **/*.pdfmarks\n\
  **/*.textclippings\n\
  **/.DS_Store\n");

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= init_test\n\
  Rene Richter <Richterrettich@gmail.com>\n\
  include::../includes/config.adoc[]\n\n\

  toc::[]\n\n");
  cleanup(&*test_project);
}

#[test]
fn it_should_create_subsequent_sections_when_in_content_root() {
  let test_project = format!("{}/section_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let result = init::init(&*test_project);
  assert!(result.is_ok());
  let mut content_path = format!("{}/content",&test_project);
  let mut section_result = section::section("blubb",&*content_path);
  assert!(section_result.is_ok());
  assert_file_content(&*format!("{}/content/1_blubb/index.adoc",test_project),
  "== blubb\n\
  include::../../includes/config.adoc[]\n\n");
  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= section_test\n\
  Rene Richter <Richterrettich@gmail.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  include::1_blubb/index.adoc[]\n\n");

  section_result = section::section("foo",&*content_path);
  assert!(section_result.is_ok());
  assert_file_content(&*format!("{}/content/2_foo/index.adoc",test_project),
  "== foo\n\
  include::../../includes/config.adoc[]\n\n");
  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= section_test\n\
  Rene Richter <Richterrettich@gmail.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  include::1_blubb/index.adoc[]\n\n\
  include::2_foo/index.adoc[]\n\n");

  content_path = format!("{}/1_blubb",&content_path);
  section_result = section::section("baz",&*content_path);
  assert!(section_result.is_ok());
  assert_file_content(&*format!("{}/content/1_blubb/1_baz/index.adoc",test_project),
  "=== baz\n\
  include::../../../includes/config.adoc[]\n\n");
  assert_file_content(&*format!("{}/content/1_blubb/index.adoc",test_project),
  "== blubb\n\
  include::../../includes/config.adoc[]\n\n\
  include::1_baz/index.adoc[]\n\n");
  cleanup(&*test_project);
}

#[test]
fn it_should_swap_positions_of_sections() {
  let test_project = format!("{}/move_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let result = init::init(&*test_project);
  assert!(result.is_ok());
  let content_path = format!("{}/content",&test_project);
  let mut section_result = section::section("blubb",&*content_path);
  assert!(section_result.is_ok());

  section_result = section::section("foo",&*content_path);
  assert!(section_result.is_ok());

  let move_result = swap_command::do_swap(1,2,&*content_path);
  assert!(move_result.is_ok());


  assert_file_content(&*format!("{}/content/1_foo/index.adoc",test_project),
  "== foo\n\
  include::../../includes/config.adoc[]\n\n");

  assert_file_content(&*format!("{}/content/2_blubb/index.adoc",test_project),
  "== blubb\n\
  include::../../includes/config.adoc[]\n\n");
  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move_test\n\
  Rene Richter <Richterrettich@gmail.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  include::1_foo/index.adoc[]\n\n\
  include::2_blubb/index.adoc[]\n\n");

  cleanup(&*test_project);
}

fn assert_file_content (file_name: &str, expected_content: &str) {
  let mut file_content = String::new();
  let mut f = fs::File::open(file_name).unwrap();
  let read_result = f.read_to_string(&mut file_content);
  assert!(read_result.is_ok());
  assert_eq!(expected_content,file_content);
}

fn cleanup(dir: &str) {
  match fs::remove_dir_all(dir) {
    Ok(_r) => (),
    Err(_err) => ()
  }
}
