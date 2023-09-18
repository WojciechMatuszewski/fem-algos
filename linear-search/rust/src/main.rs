fn main() {
    println!("{}", linear_search(1, &vec![2, 3, 4]))
}

fn linear_search<Element: PartialOrd + PartialEq>(element: Element, haystack: &[Element]) -> bool {
    for e in haystack {
        if *e == element {
            return true;
        }
    }

    return false;
}
