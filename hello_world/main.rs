fn main() {
    println!("Hello, rust");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}
