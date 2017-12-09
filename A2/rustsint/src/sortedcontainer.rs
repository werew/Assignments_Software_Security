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
        self._printtree(&self.root,0);
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Node {
            data: data,
            left: None,
            right: None,
        };




        /** TODO
        < werew> consider this method invocation: self._insertnode(&mut self.root, new_node); apparently
            Rust dislike it, since self is borrowed twice: as immutable first and as mutable later, how
            would you fix that ?
        < mbrubeck> werew: One option is, instead of having `_insertnode` be a method that takes all of
        `self`, have it be a function that takes only the values from `self` that it needs Another is to
        use interior mutability (e.g. Cell or RefCell) to safely mutate fields of `self` or `self.root`
        while they are shared And a third option is to temporarily move `self.root` out of `self`
            (replacing it with a temporary value), pass it to the method, then move it back in.
        < werew> Thank you for such complete answer :) For the first option, you mean an external function
            (not a method of this struct) right ?
        < mbrubeck> werew: Right It could be a method on some field of the struct, maybe (depending on what
                    it does)
        < werew> Totally agree thanks :)
        */

        let mut root = self.root.take();
        self._insertnode(&mut root, new_node);
        self.root = root;
    }

    pub fn contains(&self, data: T) -> bool {
        match *self._find(&self.root, data) {
            None => false,
            _    => true
        }
    }

    fn _find<'a>(&self, current: &'a Link<T>, data: T) -> &'a Link<T>{
        match current {
            &None => current,
            &Some(ref n) => {
                if n.data > data {
                    self._find(&n.left,data)
                } else if n.data < data {
                    self._find(&n.right,data)
                } else {
                    current
                }
            }
        }
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

    fn _printtree(&self, current: &Link<T>, level: usize){
        match current {
            &None => println!("{:width$}(nil)", "", width = level),
            &Some(ref n) => {
                println!("{:width$}{}", "", n.data, width = level);
                self._printtree(&n.left,level+1);
                self._printtree(&n.right,level+1);
            }
        }
    }
}

