fn main() {
    let text = String::from("hello, front end squad!");
    printer(text);
    // printer(text);
}

fn printer(text: String) {
    println!("{}", text);
}
