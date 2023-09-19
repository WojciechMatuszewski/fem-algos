fn main() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    dbg!(s.peek());

    dbg!(&s);

    s.pop();
    s.pop();

    dbg!(&s);
}

#[derive(Debug, Clone)]
struct StackNode<T: Copy> {
    value: T,
    prev: Option<Box<StackNode<T>>>,
}

#[derive(Debug)]
struct Stack<T: Copy> {
    length: usize,
    head: Option<Box<StackNode<T>>>,
}

impl<T: Copy> Stack<T> {
    fn new() -> Self {
        return Self {
            length: 0,
            head: None,
        };
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            // Implicit `copy` here with `h.value`
            Some(h) => return Some(h.value),
            None => None,
        }
    }

    fn push(&mut self, value: T) {
        self.length += 1;

        let mut node = StackNode {
            value: value,
            prev: None,
        };

        if self.head.is_none() {
            self.head = Some(Box::new(node));
            return;
        }

        node.prev = self.head.clone();
        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        self.length = usize::max(0, self.length - 1);

        let value = match self.head.clone() {
            None => None,
            Some(h) => Some(h.value),
        };

        if self.length == 0 {
            self.head = None;
            return value;
        }

        let prev = self.head.clone().unwrap().prev;
        self.head = prev;

        return value;
    }
}
