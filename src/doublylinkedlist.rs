use std::cell::{Ref, RefMut, RefCell};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    size: u32,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None, size: 0 }
    }

    pub fn push_head(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
        self.size += 1;
    }

    pub fn push_tail(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.size += 1;
    }


    pub fn push_nth(&mut self, index: u32, elem: T) {
        if index < self.size {
            let mut i = 0;
            let mut cur_node = self.head.clone();
            while i < index {
                cur_node = cur_node.map(|node| node.borrow().next.clone().unwrap());
                i += 1;
            }
            cur_node.map(|node| {
                let new_node = Node::new(elem);
                new_node.borrow_mut().next = node.borrow().next.clone();
                new_node.borrow_mut().prev = Some(node.clone());

                match node.borrow_mut().next.clone() {
                    Some(next_node) => { next_node.borrow_mut().prev = Some(new_node.clone()) }
                    None => { self.tail = Some(node.clone()) }
                }

                node.borrow_mut().next = Some(new_node);

                match node.borrow_mut().prev.clone() {
                    Some(_prev_node) => {}
                    None => { self.head = Some(node.clone()) }
                }
            });
            self.size += 1;
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.size -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_nth(&mut self, index: u32) -> Option<T> {
        if self.head.is_some() && index < self.size {
            let mut i = 0;
            let mut cur_node = self.head.clone();
            while i < index {
                cur_node = cur_node.map(|node| node.borrow().next.clone().unwrap());
                i += 1;
            }
            cur_node.map(|node| {
                match node.borrow().next.clone() {
                    Some(next_node) => {
                        next_node.borrow_mut().prev = node.borrow().prev.clone();
                    }
                    None => {
                        self.tail = node.borrow().prev.clone();
                    }
                }

                match node.borrow().prev.clone() {
                    Some(prev_node) => {
                        prev_node.borrow_mut().next = node.borrow().next.clone();
                    }
                    None => {
                        self.head = node.borrow().next.clone();
                    }
                }
                self.size -= 1;
                Rc::try_unwrap(node).ok().unwrap().into_inner().elem
            })
        }
            else {
                None
        }
    }

    pub fn reverse(&mut self) {
        for i in 1..self.size {
            let h = self.pop_head().unwrap();
            self.push_tail(h);
        }
    }


    pub fn filter(&mut self, filter_item: fn(&T) -> bool) {
        for i in 1..self.size {
            let h = self.pop_head().unwrap();
            if filter_item(&h) {
                self.push_tail(h);
            }
            else {}
        }
    }

    pub fn print(&self, print_item: fn(&T)) {
        let mut h = self.head.clone();
        loop {
            match h.clone() {
                Some(node) => {
                    print_item(&node.borrow().elem);
                    h = node.borrow().next.clone();
                }
                None => {
                    println!();
                    break;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.head.is_none() {
            true
        } else {
            false
        }
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}
