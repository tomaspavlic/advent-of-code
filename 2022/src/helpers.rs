use std::{rc::{Rc, Weak}, cell::RefCell};

pub enum Cell<T> {
    Value(T),
    Children(Vec<Rc<RefCell<Node<T>>>>),
}

pub struct Node<T> {
    cell: Cell<T>,
    parent: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new_value(v: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            cell: Cell::Value(v),
            parent: None,
        }))
    }

    pub fn new_list() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            cell: Cell::Children(vec![]),
            parent: None,
        }))
    }

    pub fn value(&self) -> &Cell<T> {
        &self.cell
    }
}

pub trait Linkable<T> {
    fn add_child(&mut self, child: Self);
    fn parent(&self) -> Self;
}

impl<T> Linkable<T> for Rc<RefCell<Node<T>>> {
    fn add_child(&mut self, node: Self) {
        if let Cell::Children(l) = &mut self.borrow_mut().cell {
            let new_node = node.clone();
            new_node.borrow_mut().parent = Some(Rc::downgrade(self));
            l.push(new_node)
        }
    }

    fn parent(&self) -> Self {
        let self_borrowed = self.borrow();
        Weak::upgrade(&self_borrowed.parent.as_ref().unwrap()).unwrap()
    }
}