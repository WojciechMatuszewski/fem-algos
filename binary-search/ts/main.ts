function binarySearch(element: number, array: Array<number>) {
  let arraySorted = [...array].sort();

  let low = 0;
  let high = arraySorted.length - 1;

  do {
    let midPoint = Math.floor(low + (high - low) / 2);

    if (element === arraySorted[midPoint]) {
      return true;
    }

    if (element > midPoint) {
      low = midPoint + 1;
    }

    high = midPoint;
  } while (low < high);

  return false;
}

console.log(binarySearch(2, [1, 2, 3, 4, 5]));
