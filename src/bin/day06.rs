use std::collections::HashMap;
use std::fs;

fn read() -> Vec<(String, String)> {
    fs::read_to_string("data/06")
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split(')');
            let left = iter.next().unwrap();
            let right = iter.next().unwrap();
            assert!(iter.next().is_none());
            (left.into(), right.into())
        })
        .collect()
}

fn pt1(input: &[(String, String)]) -> u32 {
    let mut tree = HashMap::new();
    for (parent, child) in input {
        let children = tree.entry(parent.as_str()).or_insert(Vec::new());
        children.push(child.as_str());
    }

    fn dfs(planet: &str, depth: u32, mut total: u32, tree: &HashMap<&str, Vec<&str>>) -> u32 {
        if let Some(children) = tree.get(planet) {
            for child in children {
                total = dfs(child, depth + 1, total, tree)
            }
        }
        total + depth
    }

    dfs("COM", 0, 0, &tree)
}

fn pt2(input: &[(String, String)]) -> u32 {
    let mut tree = HashMap::new();
    for (parent, child) in input {
        let children = tree.entry(parent.as_str()).or_insert(Vec::new());
        children.push(child.as_str());
    }

    fn dfs(
        planet: &str,
        target1: &str,
        target2: &str,
        tree: &HashMap<&str, Vec<&str>>,
    ) -> (Option<u32>, Option<u32>) {
        if planet == target1 {
            return (Some(0), None);
        };
        if planet == target2 {
            return (None, Some(0));
        };
        let mut got_left = None;
        let mut got_right = None;
        match tree.get(planet) {
            None => (None, None),
            Some(children) => {
                for child in children {
                    let found = dfs(child, target1, target2, tree);
                    match found {
                        // We are done, send back to base
                        (Some(l), Some(r)) => return (Some(l), Some(r)),

                        // We found one but not the other.
                        // Save this fact, and if another child finds the other, we are done
                        (Some(l), None) => got_left = Some(l + 1),
                        (None, Some(r)) => {
                            got_right = Some(r + 1);
                        }

                        // No luck this time
                        (None, None) => (),
                    }
                }
                (got_left, got_right)
            }
        }
    }

    if let (Some(l), Some(r)) = dfs("COM", "YOU", "SAN", &tree) {
        l + r - 2 // remove the 'extra counting' we did we storing the options
    } else {
        panic!("Failed")
    }
}

fn main() {
    let input = read();
    let res = pt1(&input);
    println!("{}", res);

    let res = pt2(&input);
    println!("{}", res);
}

#[test]
fn example() {
    let input: Vec<(String, String)> = vec![
        ("COM".into(), "B".into()),
        ("B".into(), "C".into()),
        ("C".into(), "D".into()),
        ("D".into(), "E".into()),
        ("E".into(), "F".into()),
        ("B".into(), "G".into()),
        ("G".into(), "H".into()),
        ("D".into(), "I".into()),
        ("E".into(), "J".into()),
        ("J".into(), "K".into()),
        ("K".into(), "L".into()),
    ];
    assert_eq!(pt1(&input), 42)
}

#[test]
fn example_6_2() {
    let input: Vec<(String, String)> = vec![
        ("COM".into(), "B".into()),
        ("B".into(), "C".into()),
        ("C".into(), "D".into()),
        ("D".into(), "E".into()),
        ("E".into(), "F".into()),
        ("B".into(), "G".into()),
        ("G".into(), "H".into()),
        ("D".into(), "I".into()),
        ("E".into(), "J".into()),
        ("J".into(), "K".into()),
        ("K".into(), "L".into()),
        ("K".into(), "YOU".into()),
        ("I".into(), "SAN".into()),
    ];
    assert_eq!(pt2(&input), 4)
}
