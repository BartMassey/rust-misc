use std::collections::LinkedList;

trait Stack<T> {
    fn spush(&mut self, v: T);
    fn spop(&mut self) -> Option<T>;
    fn sis_empty(&self) -> bool;
}

impl<'a, T: ?Sized> Stack<&'a T> for Vec<&'a T> {
    fn spush(&mut self, v: &'a T) {
        self.push(v);
    }

    fn spop(&mut self) -> Option<&'a T> {
        self.pop()
    }
    
    fn sis_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<'a, T: ?Sized> Stack<&'a T> for LinkedList<&'a T> {
    fn spush(&mut self, v: &'a T) {
        self.push_front(v);
    }

    fn spop(&mut self) -> Option<&'a T> {
        self.pop_front()
    }
    
    fn sis_empty(&self) -> bool {
        self.is_empty()
    }
}

fn dup<'a, S, T>(stack: &mut S)
    where S: Stack<&'a T>, T: 'a + ?Sized
{
    let v = stack.spop().unwrap();
    stack.spush(v);
    stack.spush(v);
}

fn test<S>(mut s: S) where S: Stack<&'static str> {
    s.spush("hello");
    dup(&mut s);
    assert_eq!(s.spop().unwrap(), "hello");
    assert_eq!(s.spop().unwrap(), "hello");
    assert!(s.sis_empty());    
}

fn main() {
    test(Vec::new());
    test(LinkedList::new());
}