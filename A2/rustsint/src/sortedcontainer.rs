use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data:  T,
    left:  Link<T>,
    right: Link<T>,
}



fn printtree<T: Display>(current: &Link<T>){
    match current {
        &None => return,
        &Some(ref n) => {
            println!("{}",n.data);
            printtree(&n.left);
            printtree(&n.right);
        }
    }
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




    pub fn insert(&mut self, data: T) {
        let new_node = Box::new(Node {
            data: data,
            left: None,
            right: None,
        });
    }



    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

