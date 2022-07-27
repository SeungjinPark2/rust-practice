//enum IpAddrKind {
//    V4(String),
//    V6(String),
//}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, // has no data associated with it at all
    Move { x: i32, y: i32 }, // has named fields like a struct does
    Write(String), // ncludes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

/*
-- same with above
struct QuitMessage; // unit struct 
struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String); // tuple struct
struct ChangeColor(i32, i32, i32); // tuple struct

//each struct has their own type - not easy to define methods on struct using impl
*/

fn main() {
//    let home = IpAddrKind::V4(String::from("127.0.0.1"));
//    let loopback = IpAddrKind::V6(String::from("::1"));
 
    let home = IpAddrKind::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddrKind::V6(String::from("::1"));

/* 
enum Option<T> {
    None,
    Some(T),
}
*/
    let some_number = Some(5);
    let some_string = some("a String");
    let absent_number = Option<i32> = None;
}

