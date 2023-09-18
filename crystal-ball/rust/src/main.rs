fn main() {
    println!(
        "{:?}",
        crystal_ball(&[false, true, true, true, true, true, true, true])
    )
}

fn crystal_ball(breaks: &[bool]) -> Option<usize> {
    let jump_amount = f32::floor(f32::sqrt(breaks.len() as f32)) as usize;

    let mut first_break = None;
    let mut iter = breaks.iter().enumerate().step_by(jump_amount);

    while let Some((i, does_break)) = iter.next() {
        if *does_break {
            first_break = Some(i);
            break;
        }
    }
    if first_break.is_none() {
        return None;
    }

    for j in 0..=jump_amount {
        let idx_to_check = j + jump_amount;
        if breaks[idx_to_check] {
            println!("{}", "im here");
            return Some(idx_to_check);
        }
    }

    return None;
}
