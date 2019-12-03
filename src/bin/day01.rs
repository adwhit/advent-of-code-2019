use std::fs;

fn read() -> Vec<i64> {
    fs::read_to_string("data/01")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn payload(base: i64) -> i64 {
    let extra = base / 3 - 2;
    if extra <= 0 {
        base
    } else {
        base + payload(extra)
    }
}

fn main() {
    let vals = read();
    let res1: i64 = vals.iter().map(|&val| payload(val) - val).sum();
    println!("{}", res1);
}

#[test]
fn test_payload() {
    assert_eq!(payload(1969) - 1969, 966);
}
