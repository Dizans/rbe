mod ch1_output;
mod ch2_prime_types;

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


fn main() {
    ch2_prime_types::practice2();
}