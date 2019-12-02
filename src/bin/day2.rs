fn read() -> Vec<usize> {
    std::fs::read_to_string("data/02.1")
        .unwrap()
        .trim()
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn run(data: &mut Vec<usize>) -> usize {
    let mut posn = 0;

    fn getvals(data: &[usize], posn: usize) -> (usize, usize, usize) {
        let v1 = data[data[posn + 1]];
        let v2 = data[data[posn + 2]];
        let target = data[posn + 3];
        (v1, v2, target)
    }

    loop {
        match data[posn] {
            1 => {
                let (v1, v2, target) = getvals(&data, posn);
                data[target] = v1 + v2;
            }
            2 => {
                let (v1, v2, target) = getvals(&data, posn);
                data[target] = v1 * v2;
            }
            99 => return data[0],
            other => panic!("Bad opcode at posn {}: {}", posn, other),
        }
        posn += 4;
    }
}

fn main() {
    let mut data = read();
    data[1] = 12;
    data[2] = 2;

    {
        let mut data = data.clone();
        let out = run(&mut data);
        println!("{}", out);
    }

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut data = data.clone();
            data[1] = noun;
            data[2] = verb;
            run(&mut data);
            if data[0] == 19690720 {
                println!("noun {} verb {} outcome {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}

#[test]
fn example() {
    let mut data = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    run(&mut data);
    assert_eq!(data, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
}
