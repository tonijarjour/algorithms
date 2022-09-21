mod singly_linked;

fn main() {
    let mut list: singly_linked::List<u8> = singly_linked::List::new();
    list.push(1).unwrap();
    list.push(2).unwrap();
    list.push(3).unwrap();
    list.push(4).unwrap();
    list.push(5).unwrap();
    list.push(6).unwrap();

    for (i, n) in list.iter().enumerate() {
        println!("{} {i}", n.borrow());
    }
    for n in list {
        println!("{}", n.borrow());
    }
}
