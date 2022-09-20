mod singly_linked;

fn main() {
    let mut list = singly_linked::List::new();
    list.push(5u8);
    list.push(6);

    println!("{:#?}", list);
}
