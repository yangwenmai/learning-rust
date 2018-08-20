extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, Rust use cargo2!");

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2018-08-13"));

    // foo " # > \ "'中国' bar
    println!(r#"foo " # > \ "'中国' bar"#);
    // println!(r"foo \" > \ "'中国' bar");
    println!("foo \"###\" bar");
    println!(r#"foo " ###" bar"#);
    println!(r#"foo " ###" bar"#);
    println!("foo \\\"###\" bar");
    println!(r####"foo \"###" bar"####);
}
