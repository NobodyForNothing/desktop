use dir_size::dir_size;
use std::fs;

#[test]
fn empty_dir() {
    fs::create_dir("empty_tmp");
    assert_eq!(dir_size("empty_tmp"), 0);
    fs::remove_dir_all("empty_tmp");
}

#[test]
fn recursive_dir() {
    fs::create_dir("rec_tmp");
    fs::write("rec_tmp/1",  "12345"); // 5 bytes
    fs::write("rec_tmp/2",  "12345"); // 5 bytes
    fs::create_dir("rec_tmp/d1"); 
    fs::create_dir("rec_tmp/d1/d1");
    fs::create_dir("rec_tmp/d1/d2");
    fs::create_dir("rec_tmp/d1/d3");
    fs::write("rec_tmp/d1/d1/1",  "123"); // 3 B
    fs::write("rec_tmp/d1/d1/2",  "23"); // 2 B
    fs::write("rec_tmp/d1/d1/3",  "");
    fs::write("rec_tmp/d1/d3/1",  "12345678901234567890"); // 20 B

    assert_eq!(dir_size("rec_tmp"), 35);
    
    fs::remove_dir_all("rec_tmp");
}

