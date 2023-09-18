type QNode<T> = {
  value: T;
  next?: QNode<T>;
};

// head -> tail
// I always think about the queue like this: tail -> head

class Queue<T> {
  public length: number;

  private head?: QNode<T> = undefined;
  private tail?: QNode<T> = undefined;

  constructor() {
    this.head = undefined;
    this.tail = undefined;
    this.length = 0;
  }

  enqueue(item: T) {
    this.length++;

    const newNode = { value: item };
    if (!this.tail) {
      this.tail = newNode;
      this.head = newNode;

      return;
    }

    /**
     * The `this.tail` and `this.head` are equal here.
     * That is because they were both assigned the same reference of the `newNode`.
     *
     * So everything we set on the `this.tail` is also copied into the `this.head`.
     * This behavior is quite confusing, but we do not have any other way of attaching ".next" to the correct node.
     */
    this.tail.next = newNode;
    this.tail = newNode;
  }

  dequeue(): T | undefined {
    if (!this.head) {
      return undefined;
    }

    const currentHead = this.head;
    this.head = this.head.next;

    this.length--;

    return currentHead.value;
  }

  peek(): T | undefined {
    return this.head?.value;
  }
}

const q = new Queue();

q.enqueue(1);
q.enqueue(2);

console.log(q);

// console.log(q.length, q.peek());

// q.dequeue();

// console.log(q.length, q.peek());
