#[derive(Debug, Clone, PartialEq)]
pub struct LinkedList<T: std::fmt::Display> {
    front: Option<Box<Link<T>>>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link<T: std::fmt::Display> {
    thing: T,
    next: Option<Box<Link<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {

    /// New instance of LinkedList
    pub fn new() -> Self {
        LinkedList {
            front: None,
            length: 0,
        }
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Adds an element to the front of the list.
    pub fn add_front(&mut self, thing: T) {
        let new_link = Box::new(Link {
            thing: thing,
            next: self.front.take(),
        });
        self.front = Some(new_link);
        self.length += 1;
    }

    /// Adds an element to the back of the list.
    pub fn add_back(&mut self, thing: T) {
        self.length += 1;

        if self.front.is_none() {
            self.front = Some(Box::new(Link::new(thing)));
            return;
        }

        let mut curr: &mut Option<Box<Link<T>>> = &mut self.front;

        while curr.is_some() && curr.as_ref().unwrap().next.is_some() {
            curr = &mut curr.as_mut().unwrap().next;
        }

        curr.as_mut().unwrap().next = Some(Box::new(Link::new(thing)));
    }

    pub fn print(&self) {
        let mut curr: &Option<Box<Link<T>>> = &self.front;        
        print!("[ ");

        while curr.is_some() {
            print!("{} ", curr.as_ref().unwrap().thing);
            curr = &curr.as_ref().clone().unwrap().next;
        }

        println!("]");
    }

}

impl<T: std::fmt::Display> Link<T> {

    /// New instance of Link
    fn new(thing: T) -> Self {
        Link {
            thing: thing,
            next: None,
        }
    }
}

fn main() {
    let mut l: LinkedList<u8> = LinkedList::new();

    l.print();
    l.add_back(2);
    l.print();
    l.add_back(3);
    l.print();
    l.add_back(4);
    l.print();
    l.add_front(1);
    l.print();
    l.add_back(5);
    l.print();
}