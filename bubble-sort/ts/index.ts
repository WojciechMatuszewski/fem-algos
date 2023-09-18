function bubbleSort(array: Array<number>) {
  let toBeSorted = [...array];

  for (let i = array.length - 1; i > 0; i--) {
    for (let j = 0; j < i; j++) {
      const current = toBeSorted[j];
      const next = toBeSorted[j + 1];

      if (current > next) {
        toBeSorted[j + 1] = current;
        toBeSorted[j] = next;
      }
    }
  }

  return toBeSorted;
}

console.log(bubbleSort([1, 3, 5, 2, 8, -1]));
