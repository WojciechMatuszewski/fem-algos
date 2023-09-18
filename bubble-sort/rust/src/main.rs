fn main() {
    let array: &mut [i32] = &mut [1, 3, 5, 2, 8, -1];
    bubble_sort(array);

    println!("{:?}", array);
}

fn bubble_sort(array: &mut [i32]) {
    for i in (0..array.len()).rev() {
        for j in 0..i {
            let current = array[j];
            let next = array[j + 1];
            if current > next {
                array[j] = next;
                array[j + 1] = current
            }
        }
    }
}
