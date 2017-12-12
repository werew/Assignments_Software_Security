use std::fmt::Display;

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


impl<T: Display + PartialOrd> SortedContainer<T> {

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


}





#[cfg(test)]
mod tests {
    use sortedcontainer::SortedContainer;


    // A basic test testing mostly a normal usage of the public API 
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


    // 
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




    // Test the deletion of a node with no children
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



    // Test the deletion of a node with two children twice (if node is root, or not)
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






    // Test the deletion of a node with two children, who each have two children as well
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




    // Test the deletion of a non-root node with two children, who each have two children as well
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
