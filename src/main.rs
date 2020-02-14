fn main() {
    println!("Hello, world!");
    if let Some(i) = std::env::args().nth(1) {
        if &i == "panic" {
            panic!("check my backtrace");
        }
    }
}
