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
    println!("Hey I just want to commit something");
    println!("this is just another one to commit");
    println!("again just committing something");
}
