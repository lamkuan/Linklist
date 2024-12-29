use linklist::LinkedList;

fn main()
{
    let mut list = LinkedList::new(1);
    list.push_front(1);
    list.push_back(2);
    list.push_back(3);

    println!("{}", list)
}
