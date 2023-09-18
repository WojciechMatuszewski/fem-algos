fn main() {
    println!("{}", binary_search(10, &vec![1, 2, 3, 4, 5, 6]))
}

fn binary_search(element: i32, array: &[i32]) -> bool {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let value = array[mid];

        if element == value {
            return true;
        }

        if element > value {
            low = mid + 1;
        }

        if element < value {
            high = mid
        }
    }

    return false;
}
