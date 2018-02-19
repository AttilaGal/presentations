fn printer(text: String) {
    println!("Hello, front-end squad!");
}

fn main() {
    let text = String::from("hello, front end squad!");
    let text2 = text;
    println!("{}", text);
    println!("{}", text2);
}

fn main() {
    let text = String::from("hello, front end squad!");
    printer(text);
    // printer(text);
}

fn printer(text: String) {
    println!("{}", text);
}
