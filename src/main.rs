mod demo02;

use std::fs;


fn main() {
    demo02::struct_demo();
}

fn http_demo() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}

fn func_demo() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));
}


fn let_demo() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2)
}

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn pi() -> f64 {
    3.1465223
}

fn not_pi() {
    3.1465223;
}

