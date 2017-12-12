use std::fmt::Display;

// A link is defined as an "Optional boxed Node"
// Nodes are boxed so that they are allocated on the heap
// We take advantage of Option in order to define empty
// links (which would be equivalent to the value None)
type Link<T> = Option<Box<Node<T>>>;


// A tree's node, containing a generic data type
// and two optional children
#[derive(Debug)]
struct Node<T> {
    data:  T,
    left:  Link<T>,
    right: Link<T>,
}


/// A binary search tree for a generic data types
/// The tree act as a set, therefore it is not possible
/// to add the same element twice
/// note: the data type used must implement the
/// Display and PartialOrd traits
pub struct SortedContainer<T> {
    root: Link<T>,
}



impl<T: Display + PartialOrd> SortedContainer<T> {

    /// Creates a new sortedcontainer
    pub fn new() -> Self {
        SortedContainer { root: None }
    }


    /// Prints the content of the tree
    /// indented according to the depth
    pub fn print(&self) {

        // Helper function: print tree recursively
        fn _printtree<T: Display>(current: &Link<T>, level: usize){
            match current {
                &None => println!("{:width$}(nil)", "", width = level),
                &Some(ref n) => {
                    println!("{:width$}{}", "", n.data, width = level);
                    _printtree(&n.left,level+1);
                    _printtree(&n.right,level+1);
                }
            }
        }

        // Use an helper function
        _printtree(&self.root,0);
    }


    /// Insert a new element into the tree
    /// If the element is already present this
    /// method does nothing
    /// @param data: data to insert into the 
    pub fn insert(&mut self, data: T) {

        // Find the link that should contain
        // the given data
        let l = self.find_pos(&data);

        // Add the data only if the link
        // is empty
        if l.is_none() {
           *l = Some(Box::new(
                Node {data: data, left: None, right: None}
           ));
        }
    }


    /// Test whether the tree contains 
    /// the given data
    /// @param data: the data in object
    /// @return true if the tree contains the
    ///     data, false otherwise
    pub fn contains(&mut self, data: T) -> bool {
        match *self.find_pos(&data) {
            None => false,
            _    => true
        }
    }


    /// Remove an element from the tree. If
    /// the element is not found nothing is done
    /// @param data: the element to remove
    pub fn erase(&mut self, data: T) {

        // Helper function: find the leftmost link starting
        // from the provided link (note: this function is
        // used so that we can target the in-order successor
        // of the element we want to erase if the latter has
        // two children)
        // @param n: the link from where to start
        // @return a mutable reference to the leftmost link
        fn _take_leftmost<'a, T>(n: &'a mut Link<T>) -> &'a mut Link<T>{
            
            let has_left_branch = n.as_ref().unwrap().left.is_some();

            if has_left_branch { 
                _take_leftmost(&mut n.as_mut().unwrap().left)
            } else { n }
        }

        // Find the target node to erase
        let target = self.find_pos(&data);
        if target.is_none() { return; } // None not found

        // Remove the target and get its content
        // note1: we use the method `take` so that we can
        // "move" the content of the target node by 
        // temporarily replacing it with a None
        // note2: we need to dereference the boxed content
        // in order to tear it apart and let the borrow 
        // checker distinguish the different fields 
        let mut target_content = *target.take().unwrap();       

        if target_content.left.is_some() && target_content.right.is_some() {
            // Node has two children ?

            // Substitute the data of the target with 
            // the content of the in-order successor
            target_content.data = {
                // Find the leftmost node on the right branch
                let leftmost = _take_leftmost(&mut target_content.right);

                // Unlink the node from the list
                let mut leftmost_content = leftmost.take().unwrap();
                *leftmost = leftmost_content.right.take();

                // Return unlinked data
                leftmost_content.data
            };
            
            // Put the target (with different data) 
            // node back on the tree
            *target = Some(Box::new(target_content));

        } else if target_content.left.is_some()  {
            // Node has only the left child 
        
            // Substitute the target with his left child
            *target = target_content.left;

        } else if target_content.right.is_some() {
            // Node has only the right child

            // Substitute the target with his right child
            *target = target_content.right;
        }
    }




    // This private function is used internally to find the
    // position of the node which should contain the specified
    // data. Note that if such node doesn't exist the returned
    // link will point to an empty leaf which represent a valid
    // position for that node (so that it can for example be
    // inserted there)
    // @param data: data to search for
    // @return a mutable reference to a link
    fn find_pos(&mut self, data: &T) -> &mut Link<T>{

        // Helper function: find specified data recursively
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

        // Use helper function
        _find(&mut self.root, data)
    }


    /*
    // This function exists for nothing but illustrative purposes:
    // it shows a different approach for inserting a node without
    // the use of the find_pos function
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
    */


}


/********************** TESTS **************************/


#[cfg(test)]
mod tests {

    use sortedcontainer::SortedContainer;


    /// A basic test testing mostly a normal usage of the public API 
    #[test]
    fn test_base_api(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();

            // A new container should be empty
            assert_eq!(sc.root.is_none(), true, "Root not empty");

            sc.insert(5); sc.insert(2); sc.insert(7); 
            sc.insert(4); sc.insert(1); sc.insert(3);
            sc.insert(6); sc.insert(9); sc.insert(8);


            // After an insertion the container should not be empty
            assert_eq!(sc.root.is_none(), false, "Root is empty");


            // The container should contain all the values inserted 
            assert!(sc.contains(1), "Does not contain value 1");
            assert!(sc.contains(2), "Does not contain value 2");
            assert!(sc.contains(3), "Does not contain value 3");
            assert!(sc.contains(4), "Does not contain value 4");
            assert!(sc.contains(5), "Does not contain value 5");
            assert!(sc.contains(6), "Does not contain value 6");
            assert!(sc.contains(7), "Does not contain value 7");
            assert!(sc.contains(8), "Does not contain value 8");
            assert!(sc.contains(9), "Does not contain value 9");


            sc.erase(5); sc.erase(3); sc.erase(8);


            // The container should not contain the erased values
            assert!(!sc.contains(5), "Contains value 5");
            assert!(!sc.contains(3), "Contains value 3");
            assert!(!sc.contains(8), "Contains value 8");


            // ..but should still contain the value not erased
            assert!(sc.contains(1), "Does not contain value 1");
            assert!(sc.contains(2), "Does not contain value 2");
            assert!(sc.contains(4), "Does not contain value 4");
            assert!(sc.contains(6), "Does not contain value 6");
            assert!(sc.contains(7), "Does not contain value 7");
            assert!(sc.contains(9), "Does not contain value 9");


            sc.erase(1); sc.erase(2); sc.erase(4);
            sc.erase(6); sc.erase(7); sc.erase(9);


            // After erasing all the values the container should be empty
            assert_eq!(sc.root.is_none(), true, "Root not empty");

    }


    /// Tests a bad usage of the public API performing a double removal
    #[test]
    fn test_double_erase(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();

            sc.insert(1); sc.insert(2);

            // The container should contain all the values inserted 
            assert!(sc.contains(1), "Does not contain value 1");
            assert!(sc.contains(2), "Does not contain value 2");

            sc.erase(1); sc.erase(1);

            // The container should not contain the erased values
            assert!(!sc.contains(1), "Contains value 1");

            // ..but should still contain the value not erased
            assert!(sc.contains(2), "Does not contain value 2");
    }




    /// Test the deletion of a node with no children
    #[test]
    fn test_erase_no_children(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();
            assert_eq!(sc.root.is_none(), true);

            sc.insert(2); 
            sc.insert(1);
            sc.insert(3);

            /*
             *  1. Test tree's initial configuration:
             *
             *          2
             *         / \
             *        1   3
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 2, "Data not equivalent"); 

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 1, "Data not equivalent");
                assert_eq!(left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left.right.is_none(), true, "Branch should be empty");

                let right = root.right.as_ref().unwrap();
                assert_eq!(right.data, 3, "Data not equivalent");
                assert_eq!(right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right.right.is_none(), true, "Branch should be empty");
            }

            sc.erase(3);

            /*
             *  2. Test tree's configuration after deleting 3:
             *
             *          2
             *         / 
             *        1  
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 2, "Data not equivalent"); 
                assert_eq!(root.right.is_none(), true, "Branch should be empty");

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 1, "Data not equivalent");
                assert_eq!(left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left.right.is_none(), true, "Branch should be empty");

            }
    }


    /// Test the deletion of a node with only one child
    #[test]
    fn test_erase_with_one_child(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();
            assert_eq!(sc.root.is_none(), true);

            sc.insert(3);
            sc.insert(1); 
            sc.insert(2);

            /*
             *  1. Test tree's initial configuration:
             *
             *          3
             *         /
             *        1  
             *         \
             *          2
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 3, "Data not equivalent"); 
                assert_eq!(root.right.is_none(), true, "Branch should be empty");

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 1, "Data not equivalent");
                assert_eq!(left.left.is_none(), true, "Branch should be empty");

                let left_right = left.right.as_ref().unwrap();
                assert_eq!(left_right.data, 2, "Data not equivalent");
                assert_eq!(left_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_right.right.is_none(), true, "Branch should be empty");
            }

            sc.erase(1);

            /*
             *  2. Test tree's configuration after deleting 1:
             *
             *          3
             *         /
             *        2  
             *         
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 3, "Data not equivalent"); 
                assert_eq!(root.right.is_none(), true, "Branch should be empty");

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 2, "Data not equivalent");
                assert_eq!(left.left.is_none(), true, "Branch should be empty");
                assert_eq!(left.right.is_none(), true, "Branch should be empty");

            }
    }



    /// Test the deletion of a node with two children twice (if node is root, or not)
    #[test]
    fn test_erase_with_two_children_1(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();
            assert_eq!(sc.root.is_none(), true);

            sc.insert(2); sc.insert(1); sc.insert(4); 
            sc.insert(3); sc.insert(5); 

            /*
             *  1. Test tree's initial configuration:
             *
             *          2
             *         / \
             *        1   4
             *           / \ 
             *          3   5
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 2, "Data not equivalent"); 

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 1, "Data not equivalent");
                assert_eq!(left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left.right.is_none(), true, "Branch should be empty");

                let right = root.right.as_ref().unwrap();
                assert_eq!(right.data, 4, "Data not equivalent");

                let right_left = right.left.as_ref().unwrap();
                assert_eq!(right_left.data, 3, "Data not equivalent");
                assert_eq!(right_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right_left.right.is_none(), true, "Branch should be empty");

                let right_right = right.right.as_ref().unwrap();
                assert_eq!(right_right.data, 5, "Data not equivalent");
                assert_eq!(right_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right_right.right.is_none(), true, "Branch should be empty");
            }


            sc.erase(4);


            /*
             *  2. Test tree's configuration after deleting 4:
             *
             *          2
             *         / \
             *        1   5
             *           /  
             *          3   
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 2, "Data not equivalent"); 

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 1, "Data not equivalent");
                assert_eq!(left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left.right.is_none(), true, "Branch should be empty");

                let right = root.right.as_ref().unwrap();
                assert_eq!(right.data, 5, "Data not equivalent");
                assert_eq!(right.right.is_none(),  true, "Branch should be empty");

                let right_left = right.left.as_ref().unwrap();
                assert_eq!(right_left.data, 3, "Data not equivalent");
                assert_eq!(right_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right_left.right.is_none(), true, "Branch should be empty");

            }


            sc.erase(2);

            
            /*
             *  3. Test tree's configuration after deleting 2 (the root):
             *
             *          3
             *         / \
             *        1   5
             *              
             *              
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 3, "Data not equivalent"); 

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 1, "Data not equivalent");
                assert_eq!(left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left.right.is_none(), true, "Branch should be empty");

                let right = root.right.as_ref().unwrap();
                assert_eq!(right.data, 5, "Data not equivalent");
                assert_eq!(right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right.right.is_none(),  true, "Branch should be empty");
            }
    }






    /// Test the deletion of a node with two children who each have two children as well
    #[test]
    fn test_erase_with_two_children_2(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();
            assert_eq!(sc.root.is_none(), true);

            sc.insert(4); sc.insert(2); sc.insert(1); 
            sc.insert(3); sc.insert(6); sc.insert(5); 
            sc.insert(7); 

            /*
             *  1. Test tree's initial configuration:
             *
             *           4
             *          / \
             *        2     6
             *       / \   / \ 
             *      1   3 5   7
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 4, "Data not equivalent"); 

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 2, "Data not equivalent");

                let left_left = left.left.as_ref().unwrap();
                assert_eq!(left_left.data, 1, "Data not equivalent");
                assert_eq!(left_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_left.right.is_none(), true, "Branch should be empty");

                let left_right = left.right.as_ref().unwrap();
                assert_eq!(left_right.data, 3, "Data not equivalent");
                assert_eq!(left_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_right.right.is_none(), true, "Branch should be empty");

                let right = root.right.as_ref().unwrap();
                assert_eq!(right.data, 6, "Data not equivalent");

                let right_left = right.left.as_ref().unwrap();
                assert_eq!(right_left.data, 5, "Data not equivalent");
                assert_eq!(right_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right_left.right.is_none(), true, "Branch should be empty");

                let right_right = right.right.as_ref().unwrap();
                assert_eq!(right_right.data, 7, "Data not equivalent");
                assert_eq!(right_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right_right.right.is_none(), true, "Branch should be empty");
            }

            
            sc.erase(4);

            /*
             *  2. Test tree's configuration after deleting 4:
             *
             *           5
             *          / \
             *        2     6
             *       / \     \ 
             *      1   3     7
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 5, "Data not equivalent"); 

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 2, "Data not equivalent");

                let left_left = left.left.as_ref().unwrap();
                assert_eq!(left_left.data, 1, "Data not equivalent");
                assert_eq!(left_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_left.right.is_none(), true, "Branch should be empty");

                let left_right = left.right.as_ref().unwrap();
                assert_eq!(left_right.data, 3, "Data not equivalent");
                assert_eq!(left_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_right.right.is_none(), true, "Branch should be empty");

                let right = root.right.as_ref().unwrap();
                assert_eq!(right.data, 6, "Data not equivalent");
                assert_eq!(right.left.is_none(),  true, "Branch should be empty");

                let right_right = right.right.as_ref().unwrap();
                assert_eq!(right_right.data, 7, "Data not equivalent");
                assert_eq!(right_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(right_right.right.is_none(), true, "Branch should be empty");
            }
    }




    /// Test the deletion of a non-root node with two children who each have two children as well
    #[test]
    fn test_erase_with_two_children_3(){

            let mut sc : SortedContainer<u32> = SortedContainer::new();
            assert_eq!(sc.root.is_none(), true);

            sc.insert(8); sc.insert(4); sc.insert(2); 
            sc.insert(1); sc.insert(3); sc.insert(6);
            sc.insert(5); sc.insert(7); 

            /*
             *  1. Test tree's initial configuration:
             *
             *             8
             *            /
             *           4
             *          / \
             *        2     6
             *       / \   / \ 
             *      1   3 5   7
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 8, "Data not equivalent"); 
                assert_eq!(root.right.is_none(),  true, "Branch should be empty");

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 4, "Data not equivalent");

                let left_left = left.left.as_ref().unwrap();
                assert_eq!(left_left.data, 2, "Data not equivalent");

                let left_left_left = left_left.left.as_ref().unwrap();
                assert_eq!(left_left_left.data, 1, "Data not equivalent");
                assert_eq!(left_left_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_left_left.right.is_none(),  true, "Branch should be empty");

                let left_left_right = left_left.right.as_ref().unwrap();
                assert_eq!(left_left_right.data, 3, "Data not equivalent");
                assert_eq!(left_left_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_left_right.right.is_none(),  true, "Branch should be empty");

                let left_right = left.right.as_ref().unwrap();
                assert_eq!(left_right.data, 6, "Data not equivalent");

                let left_right_left = left_right.left.as_ref().unwrap();
                assert_eq!(left_right_left.data, 5, "Data not equivalent");
                assert_eq!(left_right_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_right_left.right.is_none(),  true, "Branch should be empty");

                let left_right_right = left_right.right.as_ref().unwrap();
                assert_eq!(left_right_right.data, 7, "Data not equivalent");
                assert_eq!(left_right_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_right_right.right.is_none(),  true, "Branch should be empty");

            }


            sc.erase(4);

            /*
             *  2. Test tree's configuration after deleting 4:
             *
             *             8
             *            /
             *           5
             *          / \
             *        2     6
             *       / \     \ 
             *      1   3     7
             */
            {
                let root = sc.root.as_ref().unwrap();
                assert_eq!(root.data, 8, "Data not equivalent"); 
                assert_eq!(root.right.is_none(),  true, "Branch should be empty");

                let left = root.left.as_ref().unwrap();
                assert_eq!(left.data, 5, "Data not equivalent");

                let left_left = left.left.as_ref().unwrap();
                assert_eq!(left_left.data, 2, "Data not equivalent");

                let left_left_left = left_left.left.as_ref().unwrap();
                assert_eq!(left_left_left.data, 1, "Data not equivalent");
                assert_eq!(left_left_left.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_left_left.right.is_none(),  true, "Branch should be empty");

                let left_left_right = left_left.right.as_ref().unwrap();
                assert_eq!(left_left_right.data, 3, "Data not equivalent");
                assert_eq!(left_left_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_left_right.right.is_none(),  true, "Branch should be empty");

                let left_right = left.right.as_ref().unwrap();
                assert_eq!(left_right.data, 6, "Data not equivalent");
                assert_eq!(left_right.left.is_none(),  true, "Branch should be empty");

                let left_right_right = left_right.right.as_ref().unwrap();
                assert_eq!(left_right_right.data, 7, "Data not equivalent");
                assert_eq!(left_right_right.left.is_none(),  true, "Branch should be empty");
                assert_eq!(left_right_right.right.is_none(),  true, "Branch should be empty");

            }
    }

}
