extern crate asciiauthor;
use asciiauthor::*;
use std::fs;
use std::io::Read;


const  TEST_PROJECT_ROOT: &'static str = "test";

#[test]
fn it_should_split_the_path_properly() {
  let (name,path) = util::split_name("/hugo/blubb/bla");
  assert_eq!(name,"bla");
  assert_eq!(path,"/hugo/blubb");

  assert_eq!(util::split_name("bla"),("bla","."));
}

#[test]
fn it_should_create_a_propper_heading () {
  assert_eq!(util::get_heading("bla blubb foo bar"),"bla_blubb_foo_bar");
}


#[test]
fn it_should_create_a_valid_project() {
  let test_project_name = "init test";
  let test_project = format!("{}/init_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let init_result = init::init(test_project_name,"awesome@blubb.com","awesome",TEST_PROJECT_ROOT);

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
  "= init test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\

  toc::[]\n\n");
  cleanup(&*test_project);
}

#[test]
fn it_should_create_subsequent_sections_when_in_content_root() {
  let test_project_name = "section test";
  let test_project = format!("{}/section_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let result = init::init(test_project_name,"awesome@blubb.com","awesome",TEST_PROJECT_ROOT);
  assert!(result.is_ok());
  let mut content_path = format!("{}/content",&test_project);
  let mut section_result = section::section("blubb",&*content_path);
  assert!(section_result.is_ok());
  assert_file_content(&*format!("{}/content/1_blubb/index.adoc",test_project),
  "== blubb\n\
  include::../../includes/config.adoc[]\n\n");
  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= section test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_blubb\n\
  include::1_blubb/index.adoc[]\n\n");

  section_result = section::section("foo",&*content_path);
  assert!(section_result.is_ok());
  assert_file_content(&*format!("{}/content/2_foo/index.adoc",test_project),
  "== foo\n\
  include::../../includes/config.adoc[]\n\n");
  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= section test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_blubb\n\
  include::1_blubb/index.adoc[]\n\n\
  :imagesdir: 2_foo\n\
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
  //BEGIN SECTIONS\n\
  :imagesdir: 1_baz\n\
  include::1_baz/index.adoc[]\n\n");
  cleanup(&*test_project);
}

#[test]
fn it_should_swap_positions_of_sections() {
  let test_project_name = "swap test";
  let test_project = format!("{}/swap_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let result = init::init(test_project_name,"awesome@blubb.com","awesome",TEST_PROJECT_ROOT);
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
  "= swap test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_foo\n\
  include::1_foo/index.adoc[]\n\n\
  :imagesdir: 2_blubb\n\
  include::2_blubb/index.adoc[]\n\n");

  cleanup(&*test_project);
}


#[test]
fn it_should_move_a_section_to_target_position() {
  let test_project_name = "move test";
  let test_project = format!("{}/move_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let result = init::init(test_project_name,"awesome@blubb.com","awesome",TEST_PROJECT_ROOT);
  assert!(result.is_ok());
  let content_path = format!("{}/content",&test_project);
  let mut section_result = section::section("blubb",&*content_path);
  assert!(section_result.is_ok());

  section_result = section::section("foo",&*content_path);
  assert!(section_result.is_ok());
  section_result = section::section("bar",&*content_path);
  assert!(section_result.is_ok());
  section_result = section::section("bazz",&*content_path);
  assert!(section_result.is_ok());

  let mut move_result = move_command::do_move(4,1,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_bazz\n\
  include::1_bazz/index.adoc[]\n\n\
  :imagesdir: 2_blubb\n\
  include::2_blubb/index.adoc[]\n\n\
  :imagesdir: 3_foo\n\
  include::3_foo/index.adoc[]\n\n\
  :imagesdir: 4_bar\n\
  include::4_bar/index.adoc[]\n\n");


  move_result = move_command::do_move(3,1,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_foo\n\
  include::1_foo/index.adoc[]\n\n\
  :imagesdir: 2_bazz\n\
  include::2_bazz/index.adoc[]\n\n\
  :imagesdir: 3_blubb\n\
  include::3_blubb/index.adoc[]\n\n\
  :imagesdir: 4_bar\n\
  include::4_bar/index.adoc[]\n\n");

  move_result = move_command::do_move(1,3,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_bazz\n\
  include::1_bazz/index.adoc[]\n\n\
  :imagesdir: 2_blubb\n\
  include::2_blubb/index.adoc[]\n\n\
  :imagesdir: 3_foo\n\
  include::3_foo/index.adoc[]\n\n\
  :imagesdir: 4_bar\n\
  include::4_bar/index.adoc[]\n\n");


  move_result = move_command::do_move(2,3,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_bazz\n\
  include::1_bazz/index.adoc[]\n\n\
  :imagesdir: 2_foo\n\
  include::2_foo/index.adoc[]\n\n\
  :imagesdir: 3_blubb\n\
  include::3_blubb/index.adoc[]\n\n\
  :imagesdir: 4_bar\n\
  include::4_bar/index.adoc[]\n\n");

  move_result = move_command::do_move(1,4,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_foo\n\
  include::1_foo/index.adoc[]\n\n\
  :imagesdir: 2_blubb\n\
  include::2_blubb/index.adoc[]\n\n\
  :imagesdir: 3_bar\n\
  include::3_bar/index.adoc[]\n\n\
  :imagesdir: 4_bazz\n\
  include::4_bazz/index.adoc[]\n\n");

  move_result = move_command::do_move(0,2,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_blubb\n\
  include::1_blubb/index.adoc[]\n\n\
  :imagesdir: 2_foo\n\
  include::2_foo/index.adoc[]\n\n\
  :imagesdir: 3_bar\n\
  include::3_bar/index.adoc[]\n\n\
  :imagesdir: 4_bazz\n\
  include::4_bazz/index.adoc[]\n\n");

  move_result = move_command::do_move(3,15,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_blubb\n\
  include::1_blubb/index.adoc[]\n\n\
  :imagesdir: 2_foo\n\
  include::2_foo/index.adoc[]\n\n\
  :imagesdir: 3_bazz\n\
  include::3_bazz/index.adoc[]\n\n\
  :imagesdir: 4_bar\n\
  include::4_bar/index.adoc[]\n\n");

  section_result = section::section("foo_bar_bazz",&*content_path);
  assert!(section_result.is_ok());

  move_result = move_command::do_move(5,1,&*content_path);
  assert!(move_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= move test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_foo_bar_bazz\n\
  include::1_foo_bar_bazz/index.adoc[]\n\n\
  :imagesdir: 2_blubb\n\
  include::2_blubb/index.adoc[]\n\n\
  :imagesdir: 3_foo\n\
  include::3_foo/index.adoc[]\n\n\
  :imagesdir: 4_bazz\n\
  include::4_bazz/index.adoc[]\n\n\
  :imagesdir: 5_bar\n\
  include::5_bar/index.adoc[]\n\n");




  cleanup(&*test_project);
}


#[test]
fn it_should_delete_sections() {
  let test_project_name = "delete test";
  let test_project = format!("{}/delete_test",TEST_PROJECT_ROOT);
  cleanup(&*test_project);
  let result = init::init(test_project_name,"awesome@blubb.com","awesome",TEST_PROJECT_ROOT);
  assert!(result.is_ok());
  let content_path = format!("{}/content",&test_project);
  let mut section_result = section::section("blubb",&*content_path);
  assert!(section_result.is_ok());

  section_result = section::section("foo",&*content_path);
  assert!(section_result.is_ok());
  section_result = section::section("bar",&*content_path);
  assert!(section_result.is_ok());
  section_result = section::section("bazz",&*content_path);
  assert!(section_result.is_ok());

  let mut delete_result = delete_command::do_remove(3,&*content_path);
  assert!(delete_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= delete test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_blubb\n\
  include::1_blubb/index.adoc[]\n\n\
  :imagesdir: 2_foo\n\
  include::2_foo/index.adoc[]\n\n\
  :imagesdir: 3_bazz\n\
  include::3_bazz/index.adoc[]\n\n");

  delete_result = delete_command::do_remove(3,&*content_path);
  assert!(delete_result.is_ok());

  assert_file_content(&*format!("{}/content/index.adoc",test_project),
  "= delete test\n\
  awesome <awesome@blubb.com>\n\
  include::../includes/config.adoc[]\n\n\
  toc::[]\n\n\
  //BEGIN SECTIONS\n\
  :imagesdir: 1_blubb\n\
  include::1_blubb/index.adoc[]\n\n\
  :imagesdir: 2_foo\n\
  include::2_foo/index.adoc[]\n\n");


  cleanup(&*test_project);
}


fn assert_file_content (file_name: &str, expected_content: &str) {
  let mut file_content = String::new();
  let mut f = fs::File::open(file_name).unwrap();
  let read_result = f.read_to_string(&mut file_content);
  assert!(read_result.is_ok());
  assert_eq!(expected_content,&*file_content);
}

fn cleanup(dir: &str) {
  match fs::remove_dir_all(dir) {
    Ok(_r) => (),
    Err(_err) => ()
  }
}
