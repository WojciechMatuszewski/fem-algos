function crystalBall(breaks: boolean[]) {
  const jumpAmount = Math.floor(Math.sqrt(breaks.length));

  let i = jumpAmount;
  do {
    if (breaks[i]) {
      break;
    }

    i += jumpAmount;
  } while (i < breaks.length);

  i -= jumpAmount;
  for (let j = 0; j <= jumpAmount && i < breaks.length; j++, i++) {
    if (breaks[i]) {
      return i;
    }
  }

  return -1;
}

console.log(crystalBall([false, false, false, false, true, true, true]));
