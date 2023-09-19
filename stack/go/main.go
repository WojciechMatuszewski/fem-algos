package main

import (
	"fmt"
	"math"
)

func main() {
	s := Stack[int]{}
	s.Push(1)
	s.Push(2)
	s.Push(3)

	fmt.Println(s.Peek(), s.length)

	s.Pop()

	fmt.Println(s.Peek(), s.length)

	s.Pop()
	s.Pop()

	fmt.Println(s.Peek(), s.length)

}

type StackNode[T interface{}] struct {
	value T
	prev  *StackNode[T]
}

type Stack[T interface{}] struct {
	length int
	head   *StackNode[T]
}

func (s Stack[T]) New() Stack[T] {
	return Stack[T]{
		length: 0,
		head:   nil,
	}
}

func (s Stack[T]) Peek() T {
	var zeroValue T
	if s.head == nil {
		return zeroValue
	}

	return s.head.value
}

func (s *Stack[T]) Push(value T) {
	node := StackNode[T]{
		value: value,
	}

	s.length += 1

	if s.head == nil {
		s.head = &node
		return
	}

	currentHead := s.head
	s.head = &node
	node.prev = currentHead
}

func (s *Stack[T]) Pop() T {
	var zeroValue T
	s.length = int(math.Max(float64(0), float64(s.length-1)))

	if s.head == nil {
		return zeroValue
	}

	if s.length == 0 {
		value := s.head.value
		s.head = nil
		return value
	}

	currentHead := s.head
	s.head = currentHead.prev
	return currentHead.value
}
