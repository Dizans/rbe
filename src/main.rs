mod ch1_output;
mod ch2_prime_types;
mod ch3_self_defined_types;

#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct Point {
    x: f32,
    y: f32,
}


fn main() {
    use ch3_self_defined_types::List;
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
