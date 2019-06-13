#[derive(Debug)]
struct Tree<K, V>{
    key: K,
    value: V,
    left: Option<Box<Tree<K, V>>>,
    right: Option<Box<Tree<K, V>>>,
}

impl<K, V> Tree<K, V>{
    fn new(k: K, v: V) -> Self{
        Tree{key: k, value: v, left: None, right: None}
    }

    fn get_current(&self) -> (&K, &V){
        (&self.key, &self.value)
    }

    fn set_left(&mut self, child: Tree<K, V>) {
        self.left = Some(Box::new(child));
    }

    fn set_right(&mut self, child: Tree<K, V>) {
        self.right = Some(Box::new(child));
    }

    fn get_left(&mut self) -> &mut Option<Box<Tree<K, V>>>{
        &mut self.left
    }

    fn get_right(&mut self) -> &mut Option<Box<Tree<K, V>>>{
        &mut self.right
    }
}

fn main() {
    println!("Hello, world!");
    let mut foo = Tree::new(32,41);
    foo.set_right(Tree::new(2,7));
    println!("{:?}", foo.get_right().as_ref().unwrap().get_current());
    foo.get_right().as_mut().unwrap().set_right(Tree::new(2,7));
    println!("{:?}", foo.get_right().as_mut().unwrap().get_right().as_ref().unwrap().get_current());

}
