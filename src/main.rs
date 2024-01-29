use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

struct Node<T:Debug>{
    data:T,
    next:Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl <T:Debug> Node<T>{
    fn new(val:T) -> Self{
        Self { data: val, next: None }
    }
}

struct List<T:Debug>{
    head:Link<T>,
    tail:Link<T>
}

impl <T:Debug> List<T>{
    fn new() -> Self{
        Self { head: None, tail: None }
    }

    fn insert(&mut self,val:T){
        let mut new_node = Rc::new(RefCell::new(Node::new(val)));

        match self.tail.take() {
            None => {
                self.head = Some(new_node);
                self.tail = self.head.clone();
            },
            Some(t) => {
                t.borrow_mut().next = Some(new_node);
                self.tail = t.borrow_mut().next.clone();
            }
        }
    }

    fn push(&mut self,val:T){
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        if self.head.is_none(){
            self.tail = Some(new_node.clone());
        }
        new_node.borrow_mut().next = self.head.take();
        self.head = Some(new_node.clone());
        
    }

    fn pop(&mut self){
        if self.head.is_none(){
            return;
        }
        
        self.head = self.head.take().unwrap().borrow_mut().next.take(); // LOOK AT THIS MONSTROSITY
        
    }

    fn debug_print(&mut self){
        
        while let Some(i) = self.head.clone() {
            println!("{:?}",i.borrow().data);
            self.head = i.borrow().next.clone();    // TO MUCH HECTIC FIGHTING RUNTIME BORROW CHECKER THROUGH RefCell
        }
    }
}





#[test]
fn main2(){
    let mut a = List::<i32>::new();
    a.push(89);
    a.push(891);
    a.push(10);
    a.push(1001);
    a.pop();

    a.debug_print();
}

fn main(){
    let mut a = List::<i32>::new();
    a.push(89);
    a.push(891);
    a.push(10);
    a.push(1001);
    a.pop();

    a.debug_print();
}