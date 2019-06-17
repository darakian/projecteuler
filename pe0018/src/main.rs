use std::fs::File;
use std::collections::VecDeque;
use std::io::Read;

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

    fn set_value(mut self, val: V) -> (){
        self.value = val;
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

    fn get_children(&mut self) -> (&mut Option<Box<Tree<V>>>, &mut Option<Box<Tree<V>>>) {
        (&mut self.left, &mut self.right)
    }
}

fn main() {
    let mut file = File::open("nums.txt").unwrap();
    let mut nums = String::new();
    file.read_to_string(&mut nums);
    let mut nodes = VecDeque::new();
    let mut tree_vec = Vec::new();
    let mut first = Some(Box::new(Tree::new(0)));
    nodes.push_back(&mut first);
    for token in nums.split_whitespace(){
        let number = token.parse::<u32>().unwrap();
        let mut node = nodes.pop_front().unwrap().unwrap();
        node.set_value(number);
        node.set_left(Tree::new(0));
        node.set_right(Tree::new(0));
        let (mut left_ref, mut right_ref) = node.get_children();
        nodes.push_back(left_ref);
        nodes.push_back(right_ref);
        println!("{:?}", number);
    }
    // for i in 1..10 {
    //     let mut node = nodes.pop_front().as_mut().unwrap();
    //     node.set_value(i);
    //     node.set_left(Tree::new(0));
    //     node.set_right(Tree::new(0));
    //     let (left_ref, right_ref) = node.get_children();
    //     let mut left_ref = left_ref.as_mut().unwrap();
    //     let mut right_ref = right_ref.as_mut().unwrap();
    //     nodes.push_back(left_ref);
    //     nodes.push_back(right_ref);

    // }
    // let mut foo = Tree::new(75);
    // let (left_ref, right_ref) = foo.get_children();
    // let mut left_ref = left_ref.as_mut();
    // let mut right_ref = right_ref.as_mut();
    // nodes.push_back(left_ref);
    // nodes.push_back(right_ref);

    // println!("{:?}", foo.get_value());
    // foo.set_right(Tree::new(7));
    // println!("{:?}", foo.get_right().as_ref().unwrap().get_value());
    // foo.get_right().as_mut().unwrap().set_right(Tree::new(2));
    // println!("{:?}", foo.get_right().as_mut().unwrap().get_right().as_ref().unwrap().get_value());
    // foo.get_right().as_mut().unwrap().get_right().as_mut().unwrap().set_right(Tree::new(3));

}
