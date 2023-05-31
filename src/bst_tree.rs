#[derive(PartialEq)]
pub struct BST {
    data: i32,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

impl BST {
    pub fn new(data: i32) -> BST {
        BST {
            data: data,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, data: i32) {
        if data < self.data {
            match self.left {
                Some(ref mut left) => left.insert(data),
                None => self.left = Some(Box::new(BST::new(data))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(data),
                None => self.right = Some(Box::new(BST::new(data))),
            }
        }
        println!("Inserted {:?}", data);
    }

    //TODO
    pub fn modify(&mut self, newValue: i32) {
        self.data = newValue;
        println!("Node updated");
    }

    pub fn print(&self) {
        //Print in format "Data: {:?} -- LEFT: {:?} || RIGHT: {:?}"
        match self.left {
            Some(ref left) => left.print(),
            None => (),
        }
        println!("NODE: {:?}", self.data);
        match self.right {
            Some(ref right) => right.print(),
            None => (),
        }
    }

}
