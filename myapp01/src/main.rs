fn main() {
    let greeting = "Hello, Rust learner!";
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    println!("Welcome to myapp01 version {}, name {}", version, name);

    println!("{}", "Hello, Rust learner!");
    println!("{}", greeting);

    let number = 42;
    println!("The answer is: {}", number);

    println!("{:?}", std::env::vars());
}