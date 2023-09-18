use std::fmt::Debug;

fn main() {
    let mut q: Queue<usize> = Queue::new();
    q.enqueue(10);
    q.enqueue(2);

    dbg!(q);
}

#[derive(Clone, Debug)]
struct QNode<T: Clone + Copy + Debug> {
    value: T,
    next: Option<Box<QNode<T>>>,
}

#[derive(Debug)]
struct Queue<T: Clone + Copy + Debug> {
    head: Option<QNode<T>>,
    tail: Option<QNode<T>>,
    length: usize,
}

impl<T: Clone + Copy + Debug> Queue<T> {
    fn new() -> Self {
        return Queue {
            head: None,
            tail: None,
            length: 0,
        };
    }

    fn peek(self) -> Option<T> {
        dbg!(&self);
        return match self.head {
            Some(h) => Some(h.value),
            None => None,
        };
    }

    fn enqueue(&mut self, value: T) {
        self.length += 1;

        if let None = self.tail {
            self.head = Some(QNode {
                value: value.clone(),
                next: None,
            });
            // In Rust, I could not find any way to have the same "copying" semantics as we have in Go and JS.
            // Probably something with lifetimes, but I cannot figure out what.

            // self.tail = self.head;

            // self.tail = Some(QNode {
            //     value: value,
            //     next: None,
            // });

            return;
        }

        let new_node = QNode {
            value: value,
            next: None,
        };

        self.tail.as_mut().expect("").next = Some(Box::new(new_node.clone()));
        self.tail = Some(new_node);
    }

    fn dequeue(&mut self) -> Option<T> {
        return None;
        // if self.head.is_none() {
        //     return None;
        // }
        // self.length -= 1;

        // let current_head = self.head.clone().unwrap();

        // let next = match self.head.clone().expect("").next {
        //     None => None,
        //     Some(h) => Some(*h),
        // };

        // self.head = next;

        // return Some(current_head.value);
    }
}
