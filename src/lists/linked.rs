// Project algorithms-rs
// Create by VenmoSnake 2020/7/13 17:53
//

use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Node<V> {
    value: V,
    next: Option<Box<Node<V>>>,
}

impl<V> Node<V> {
    pub fn new(val: V) -> Self {
        Self {
            value: val,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn push_front(&mut self, ele: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(ele)));
        } else {
            let mut node = Box::new(Node::new(ele));
            node.next = self.head.take();
            self.head = Some(node);
        }
        self.size += 1;
    }

    pub fn push_back(&mut self, ele: T) {
        if let Some(mut node) = self.head.as_mut() {
            while let Some(ref mut n) = node.next {
                node = n;
            }
            node.next = Some(Box::new(Node::new(ele)));
        } else {
            self.head = Some(Box::new(Node::new(ele)));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        // let mut res = match self.head.as_mut() {
        //     Some(node) => {
        //         let mut cur = node;
        //         while let Some(n) = cur.next{
        //             cur = n.next.as_mut().unwrap();
        //         }
        //         Some(cur)
        //     }
        //     None => None
        // };
        // let cur = res.as_mut().unwrap().next.take();
        // Some(cur.unwrap().value)
        None
    }
}

impl<T: Debug> LinkedList<T> {
    pub fn print(&self) {
        let mut node = self.head.as_ref();
        while let Some(n) = node {
            println!("{:?}", n.value);
            node = n.next.as_ref();
        }
    }
}


#[test]
fn test() {
    let mut ls = LinkedList::new();
    ls.push_back(10);
    ls.push_back(50);
    ls.push_back(60);

    ls.push_front(5);

    ls.pop();
    ls.print()
}