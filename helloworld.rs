
fn main() {
    let s = String::from("hello");  // s is a new String instance, and it is now the owner of the string data

    let t = s;  // t is now the owner of the string data. s is no longer valid, and trying to use it will result in a compile-time error

    let u = t.clone();  // u is now a new owner of a copy of the string data. Both t and u are valid, and can be used independently of each other.
    println!("t value after clone() = {}",t);

    println!("U value after clone() = {}",u);
}
