# Frontend Masters - The Last Algorithms Course You'll Need

> [You can find the link to the course here](https://frontendmasters.com/courses/algorithms/)

Watching the course and writing the algorithms in Go, TS and Rust.

## Array vs Linked List

- If you are pushing or popping, the _linked list_ works pretty well.

  - You do not want to use an array here, since if you did, you would have to shift all the elements by 1.

- With the _array_, you have access to every item at a given index.

  - To get an item at a given index with _linked list_, you would have to traverse the whole list up to certain point.

- **The _array_ has to allocate all the memory upfront**.

  - That is not the case with _linked list_. The _linked list_ can grow as the items are added.

  - Of course, in most programming languages, the _arrays_ also grow when you add items to them.

    - **An _array_ that grows is called _array list_ or a _vector_**.
