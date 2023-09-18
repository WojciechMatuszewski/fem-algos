package main

import "github.com/davecgh/go-spew/spew"

func main() {
	queue := Queue[int]{}.New()
	queue.Enqueue(1)
	queue.Enqueue(2)
}

type QNode[T interface{}] struct {
	value T
	next  *QNode[T]
}

type Queue[T interface{}] struct {
	length int
	head   *QNode[T]
	tail   *QNode[T]
}

func (q Queue[T]) New() Queue[T] {
	return Queue[T]{
		length: 0,
	}
}

func (q Queue[T]) Peek() T {
	var zeroValue T
	if q.head == nil {
		return zeroValue
	}

	return q.head.value
}

func (q *Queue[T]) Enqueue(value T) {
	q.length++

	newNode := &QNode[T]{value: value}

	if q.head == nil {
		q.head = newNode
		q.tail = newNode
		return
	}

	/*
		Here, we are using the same "copy" semantics we did in TS version.
	*/
	q.tail.next = newNode
	q.tail = newNode

	spew.Dump("head", q.head, "tail", q.tail)

}

func (q *Queue[T]) Dequeue() T {
	var zeroValue T
	if q.head == nil {
		return zeroValue
	}

	q.length--

	head := q.head
	q.head = q.head.next

	return head.value
}
