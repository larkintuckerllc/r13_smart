use crate::List::{Cons, Nil};
use crate::ListTwo::{ConsTwo, NilTwo};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
    
    let a = Rc::new(ConsTwo(5,
        Rc::new(ConsTwo(10,
             Rc::new(NilTwo)))));
    let b = ConsTwo(3, Rc::clone(&a));
    let c = ConsTwo(4, Rc::clone(&a));

     let counter = Counter::new();
     println!("{}", counter.value.borrow()); // 0
     counter.increment();
     println!("{}", counter.value.borrow()); // 1
}

enum List {
    Cons(i32, Box<List>),
    Nil
}

enum ListTwo {
    ConsTwo(i32, Rc<ListTwo>),
    NilTwo
}

struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            value: RefCell::new(0)
        }
    }

    fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }
}