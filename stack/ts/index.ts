type SNode<T> = {
  value: T;
  prev?: SNode<T>;
};

export class Stack<T> {
  public length: number;

  private head?: SNode<T> = undefined;

  constructor() {
    this.head = undefined;
    this.length = 0;
  }

  push(value: T): T {
    const newNode: SNode<T> = {
      value
    };

    this.length++;

    if (!this.head) {
      this.head = newNode;
      return value;
    }

    const head = this.head;
    newNode.prev = head;

    this.head = newNode;

    return value;
  }

  pop(): T | undefined {
    if (!this.head) {
      return undefined;
    }

    this.length = Math.max(0, this.length - 1);
    if (this.length == 0) {
      const headValue = this.head.value;
      this.head = undefined;
      return headValue;
    }

    const currentHead = this.head;
    this.head = currentHead.prev;

    return currentHead.value;
  }

  peek(): T | undefined {
    return this.head?.value;
  }
}

const s = new Stack();

s.push(1);
s.push(2);
s.push(3);

console.log(s.peek());

console.log(s.pop());
