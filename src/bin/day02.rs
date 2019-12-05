use advent2019::Machine;

fn read() -> Vec<usize> {
    std::fs::read_to_string("data/02")
        .unwrap()
        .trim()
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    let data = read();
    let mut machine = Machine::init(data.clone(), 12, 2);

    let res = machine.run();

    println!("{}", res);

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut machine = Machine::init(data.clone(), noun, verb);
            let res = machine.run();
            if res == 19690720 {
                println!("noun {} verb {} outcome {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}

#[test]
fn example() {
    let data = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut machine = Machine::new(data);
    machine.run();
    assert_eq!(machine.state(), &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
}
