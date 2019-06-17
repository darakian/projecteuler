use std::fs::File;
use std::collections::VecDeque;

#[derive(Debug)]
struct Tree<V>{
    value: V,
    left: Option<Box<Tree<V>>>,
    right: Option<Box<Tree<V>>>,
}

impl<V> Tree<V>{
    fn new(v: V) -> Self{
        Tree{value: v, left: None, right: None}
    }

    fn get_value(&self) -> &V{
        &self.value
    }

    fn set_left(&mut self, child: Tree<V>) {
        self.left = Some(Box::new(child));
    }

    fn set_right(&mut self, child: Tree<V>) {
        self.right = Some(Box::new(child));
    }

    fn get_left(&mut self) -> &mut Option<Box<Tree<V>>>{
        &mut self.left
    }

    fn get_right(&mut self) -> &mut Option<Box<Tree<V>>>{
        &mut self.right
    }
}

fn main() {
    let file = File::open("nums.txt").unwrap();
    let mut foo = Tree::new(75);
    let mut nodes = VecDeque::new();
    let left_ref = foo.get_left().as_ref();
    let right_ref = foo.get_right().as_ref();
    nodes.push_back(left_ref);
    nodes.push_back(right_ref);

    let mut int_queue = Vec::new();
    int_queue.push(foo);
    println!("{:?}", int_queue[0].get_value());
    int_queue[0].set_right(Tree::new(7));
    println!("{:?}", int_queue[0].get_right().as_ref().unwrap().get_value());
    int_queue[0].get_right().as_mut().unwrap().set_right(Tree::new(2));
    println!("{:?}", int_queue[0].get_right().as_mut().unwrap().get_right().as_ref().unwrap().get_value());
    int_queue[0].get_right().as_mut().unwrap().get_right().as_mut().unwrap().set_right(Tree::new(3));

}
