mod linked_list;
fn main() {
    let mut list = linked_list::Node::new(1);
    list.add(2);
    list.add(3);
    list.print();
}
