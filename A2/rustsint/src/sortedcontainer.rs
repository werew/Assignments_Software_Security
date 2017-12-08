use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data:  T,
    left:  Link<T>,
    right: Link<T>,
}





pub struct SortedContainer<T> {
    root: Link<T>,
}

impl<T: Display + PartialOrd> SortedContainer<T> {

    pub fn new() -> Self {
        SortedContainer { root: None }
    }

    pub fn print(&self) {
        self._printtree(&self.root);                
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Node {
            data: data,
            left: None,
            right: None,
        };

        let mut root = self.root.take();
        self._insertnode(&mut root, new_node);
        self.root = root;
    }









    fn _insertnode(&self, current: &mut Link<T>, new_node: Node<T>){
        match current {
            &mut None => {
                // Insert Node
                *current = Some(Box::new(new_node));
            }

            &mut Some(ref mut n) => {
                if n.data > new_node.data {
                    self._insertnode(&mut n.left, new_node);
                } else if n.data < new_node.data {
                    self._insertnode(&mut n.right, new_node);
                } else {
                    // Node is already present
                    return;
                }
            }
        }
    }

    // TODO ident-depth
    fn _printtree(&self, current: &Link<T>){
        match current {
            &None => println!("(Nil)"),
            &Some(ref n) => {
                println!("{}",n.data);
                self._printtree(&n.left);
                self._printtree(&n.right);
            }
        }
    }
}

