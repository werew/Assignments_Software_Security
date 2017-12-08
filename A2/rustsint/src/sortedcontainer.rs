

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data:  T,
    left:  Link<T>,
    right: Link<T>,
}




pub struct SortedContainer<T> {
    root: Link<T>,
}

impl<T> SortedContainer<T> {
    pub fn new() -> Self {
        SortedContainer { root: None }
    }

    pub fn print(&self) {

    }

    pub fn insert(&self) {
    }
}

