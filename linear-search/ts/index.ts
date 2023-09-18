function linearSearch<E, A extends Array<E>>(array: A, element: E) {
  for (let i = 0; i < array.length; i++) {
    if (array[i] === element) {
      return true;
    }
  }

  return false;
}
