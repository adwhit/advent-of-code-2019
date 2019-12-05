fn digits(mut v: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    loop {
        digits.push(v % 10);
        v = v / 10;
        if v == 0 {
            break;
        }
    }
    assert_eq!(digits.len(), 6);
    digits
}

fn is_valid_1(v: &u32) -> bool {
    let digits = digits(*v);
    let mut has_adjacent = false;
    let mut is_increasing = true;
    for window in digits.windows(2) {
        let is_adjacent = window[0] == window[1];
        has_adjacent |= is_adjacent;
        is_increasing &= window[0] >= window[1];
    }
    has_adjacent && is_increasing
}

fn is_valid_2(v: &u32) -> bool {
    let digits = digits(*v);
    let mut has_run_of_two = false;
    let mut current_run = 1;
    let mut is_increasing = true;
    for window in digits.windows(2) {
        is_increasing &= window[0] >= window[1];
        let adjacent = window[0] == window[1];
        if adjacent {
            current_run += 1;
        } else {
            // run has finished
            if current_run == 2 {
                has_run_of_two = true;
            }
            current_run = 1;
        }
    }
    if current_run == 2 {
        has_run_of_two = true;
    }
    is_increasing && has_run_of_two
}

fn main() {
    let ct = (240920..=789857).filter(is_valid_1).count();
    println!("Pt1: Valid vals: {}", ct);

    let ct = (240920..=789857).filter(is_valid_2).count();
    println!("Pt2: Valid vals: {}", ct);
}

#[test]
fn example1() {
    assert!(is_valid_1(&111111));
    assert!(is_valid_1(&112345));
    assert!(!is_valid_1(&223450));
    assert!(!is_valid_1(&123789));
}

#[test]
fn example2() {
    assert!(is_valid_2(&112233));
    assert!(!is_valid_2(&123444));
    assert!(is_valid_2(&111122));
    assert!(is_valid_2(&112333));
}
