use std::collections::BTreeMap;

enum Direction {
    Up, Down, Left, Right
}

fn parse(data: &str) -> (Vec<(Direction, i32)>, Vec<(Direction, i32)>) {

    fn parse_entry(entry: &str) -> (Direction, i32) {
        use Direction::*;
        let dirn = match entry.chars().nth(0).unwrap() {
            'U' => Up,
            'D' => Down,
            'R' => Right,
            'L' => Left,
            other => panic!("Bad char: {}", other)
        };
        let dist: i32 = entry[1..].parse().unwrap();
        (dirn, dist)
    }

    let mut iter = data
        .lines().map(|line| {
            line
                .trim()
                .split(',')
                .map(parse_entry)
                .collect::<Vec<_>>()
        });
    (iter.next().unwrap(), iter.next().unwrap())
}

fn get_route(route: &[(Direction, i32)]) -> BTreeMap<(i32, i32), u32> {
    use Direction::*;
    let mut track = BTreeMap::new();
    let mut totdist = 0;
    let mut posx = 0;
    let mut posy = 0;
    for (dirn, dist) in  route {
        for _ in 0..*dist {
            totdist += 1;
            match dirn {
                Up => {
                    posy += 1
                }
                Down => {
                    posy -= 1
                }
                Left => {
                    posx -= 1
                }
                Right => {
                    posx += 1
                }
            }
            track.insert((posx, posy), totdist);
        }
    }
    track
}

fn get_min_dist(route1: &[(Direction, i32)], route2: &[(Direction, i32)]) -> (u32, u32) {
    let route1 = get_route(&route1);
    let route2 = get_route(&route2);
    let mut min_manhat = std::u32::MAX;
    let mut min_route = std::u32::MAX;
    for (posn, steps1) in route1.iter() {
        if let Some(steps2) = route2.get(posn) {
            let manhat = (posn.0.abs() + posn.1.abs()) as u32;
            if manhat < min_manhat { min_manhat = manhat };
            let route = steps1 + steps2;
            if route < min_route { min_route = route };
        }
    }
    (min_manhat, min_route)
}


fn main() {
    let file = std::fs::read_to_string("data/03").unwrap();
    let (line1, line2) = parse(&file);
    let mins = get_min_dist(&line1, &line2);
    println!("Pt 1: {}", mins.0);
    println!("Pt 2: {}", mins.1);
}

#[test]
fn example() {
    let file = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let (line1, line2) = parse(file);
    assert_eq!(get_min_dist(&line1, &line2), (6, 30));

    let file = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let (line1, line2) = parse(file);
    assert_eq!(get_min_dist(&line1, &line2), (159, 610));
}
