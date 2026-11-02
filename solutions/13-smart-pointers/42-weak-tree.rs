use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(name: &str) -> Rc<Node> {
        Rc::new(Node {
            name: name.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    fn parent(&self) -> Option<Rc<Node>> {
        self.parent.borrow().upgrade()
    }

    /// Path from the root down to this node, e.g. "root/usr/bin".
    fn path(&self) -> String {
        match self.parent() {
            Some(p) => format!("{}/{}", p.path(), self.name),
            None => self.name.clone(),
        }
    }

    fn depth(&self) -> usize {
        self.parent().map_or(0, |p| p.depth() + 1)
    }

    fn count_descendants(&self) -> usize {
        self.children
            .borrow()
            .iter()
            .map(|c| 1 + c.count_descendants())
            .sum()
    }
}

fn add_child(parent: &Rc<Node>, child: Rc<Node>) {
    *child.parent.borrow_mut() = Rc::downgrade(parent);
    parent.children.borrow_mut().push(child);
}

fn print_counts(label: &str, node: &Rc<Node>) {
    println!(
        "{label}: strong = {}, weak = {}",
        Rc::strong_count(node),
        Rc::weak_count(node)
    );
}

fn main() {
    let leaf = Node::new("bin");
    print_counts("leaf (alone)", &leaf);

    {
        let root = Node::new("root");
        let usr = Node::new("usr");
        add_child(&root, Rc::clone(&usr));
        add_child(&usr, Rc::clone(&leaf));

        println!("leaf path = {}", leaf.path());
        println!("leaf depth = {}", leaf.depth());
        println!("root descendants = {}", root.count_descendants());
        print_counts("root", &root);
        print_counts("usr", &usr);
        print_counts("leaf", &leaf);
    }

    // root and usr are gone: the Weak parent link no longer upgrades.
    println!("leaf parent after scope = {:?}", leaf.parent().map(|p| p.name.clone()));
    print_counts("leaf (after scope)", &leaf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_links_and_path() {
        let root = Node::new("a");
        let b = Node::new("b");
        let c = Node::new("c");
        add_child(&root, Rc::clone(&b));
        add_child(&b, Rc::clone(&c));
        assert_eq!(c.path(), "a/b/c");
        assert_eq!(c.depth(), 2);
        assert_eq!(root.count_descendants(), 2);
    }

    #[test]
    fn counts_reflect_weak_parent_links() {
        let root = Node::new("root");
        let child = Node::new("child");
        add_child(&root, Rc::clone(&child));
        assert_eq!(Rc::strong_count(&root), 1);
        assert_eq!(Rc::weak_count(&root), 1);
        assert_eq!(Rc::strong_count(&child), 2);
        assert_eq!(Rc::weak_count(&child), 0);
    }

    #[test]
    fn dropping_parent_frees_it() {
        let child = Node::new("child");
        let weak_root;
        {
            let root = Node::new("root");
            weak_root = Rc::downgrade(&root);
            add_child(&root, Rc::clone(&child));
            assert!(child.parent().is_some());
        }
        assert!(weak_root.upgrade().is_none());
        assert!(child.parent().is_none());
        assert_eq!(Rc::strong_count(&child), 1);
    }
}
