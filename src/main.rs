mod singly_linked;

fn main() {
    let mut list: singly_linked::List<u8> = singly_linked::List::new();
    list.push(1).unwrap();
    list.push(2).unwrap();
    list.push(3).unwrap();
    list.push(4).unwrap();
    list.push(5).unwrap();
    list.push(6).unwrap();

    list.get(5).unwrap().borrow_mut().value = 13;
    println!("{}", list.get(2).unwrap().borrow());
    println!("{}", list.get(4).unwrap().borrow());
    println!("{}", list.get(5).unwrap().borrow());
}
