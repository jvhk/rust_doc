mod match_operator;

enum IpAddrKind{
    V4,
    V6,
}

/* Enum Message:

    Quit has no data associated with it at all.
    Move has named fields like a struct does.
    Write includes a single String.
    ChangeColor includes three i32 values.

*/
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

struct Quit;
struct Move{
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32,i32);

impl Message{
    fn call(&self)  {
        println!("{:#?}", &self);
    }
}

// Rust not have null value
enum  Option<T>  {
    None,
    Some(T),
}

fn main() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback =  IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello call"));
    m.call();    


    //Enum option
    let some_number = Some(5);
    let some_string = Some("some string");

    //let absent_number: Option<i32> = Some(8); mismatch error
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value{
        println!("three");
    }
} 
