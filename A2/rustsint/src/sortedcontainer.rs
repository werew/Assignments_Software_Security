use std::fmt::Display;
use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data:  T,
    left:  Link<T>,
    right: Link<T>,
}





pub struct SortedContainer<T> {
    root: Link<T>,
}


impl<T: Debug + Display + PartialOrd> SortedContainer<T> {

    pub fn new() -> Self {
        SortedContainer { root: None }
    }



    pub fn print(&self) {
        self._printtree(&self.root,0);
    }



    pub fn insert(&mut self, data: T) {
        let r = self.find(&data);

        if r.is_none() {
           *r = Some(
                Box::new(Node {
                   data: data,
                   left: None,
                   right: None,
                })
           );
        }
    }



    pub fn contains(&mut self, data: T) -> bool {
        match *self.find(&data) {
            None => false,
            _    => true
        }
    }


    pub fn erase(&mut self, data: T) {

        fn _take_leftmost<'a, T>(n: &'a mut Link<T>) -> &'a mut Link<T>{
                
               let has_left_branch = n.as_ref().unwrap().left.is_some();

               if has_left_branch { 
                   _take_leftmost(&mut n.as_mut().unwrap().left)
               } else { n }
        }



        let target = self.find(&data);
        if target.is_none() { return; } // None not found

        let mut target_content = *target.take().unwrap();       // Note, dereference the box!
       

        if target_content.left.is_some() && target_content.right.is_some() {

            target_content.data = {
                /* Unlink */
                let leftmost = _take_leftmost(&mut target_content.right);
                let mut leftmost_content = leftmost.take().unwrap();
                *leftmost = leftmost_content.right.take();

                /* Return unlinked data */
                leftmost_content.data
            };
            
            /* Put back */
            *target = Some(Box::new(target_content));


        } else if target_content.left.is_some()  {
            *target = target_content.left;

        } else if target_content.right.is_some() {
            *target = target_content.right;
        }
    }





    fn find(&mut self, data: &T) -> &mut Link<T>{

        fn _find<'a, T: PartialOrd>( current: &'a mut Link<T>, data: &T) -> &'a mut Link<T>{

            enum Direction { Right, Left, Arrived }

            let val = match *current {
                None => Direction::Arrived,
                Some(ref n) => {
                    if      n.data > *data { Direction::Left    }
                    else if n.data < *data { Direction::Right   }
                    else                   { Direction::Arrived }
                }
            };

            match val {
                Direction::Left    => _find(&mut current.as_mut().unwrap().left, data),
                Direction::Right   => _find(&mut current.as_mut().unwrap().right, data),
                Direction::Arrived => current,
            }
        }

        _find(&mut self.root, data)
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




    /************* OLD *************************/
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
        /*
        let new_node = Node {
            data: data,
            left: None,
            right: None,
        };
        */




        /* TODO
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

        /*
        let mut root = self.root.take();
        self._insertnode(&mut root, new_node);
        self.root = root;
        */

}

