struct Person {
    name: &'static str,
    age: u64,
    income: u32,
}

fn main() {
    let i = 32;
    if i > 0 {
        println!("{} is greater than 0.", i);
    }
}
