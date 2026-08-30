fn main() {
    println!("{}", say_hello_world())
}

fn say_hello_world() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_say_hello_world() {
    assert_eq!("Hello, World!", say_hello_world());
}