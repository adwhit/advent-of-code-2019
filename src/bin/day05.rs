use advent2019::Machine;

fn main() {
    let mut machine = Machine::from_file("data/05");
    machine.run();
}

#[test]
fn example1() {
    let data = vec![1002, 4, 3, 4, 33];
    let mut machine = Machine::new(data);
    machine.run();
    assert_eq!(machine.state(), &[1002, 4, 3, 4, 99]);
}

#[test]
fn example2() {
    let data = vec![1101, 100, -1, 4, 0];
    let mut machine = Machine::new(data);
    machine.run();
    assert_eq!(machine.state(), &[1101, 100, -1, 4, 99]);
}


// #[test]
// fn example_5_2() {

//     // The below example program uses an input instruction to ask for a single number.
//     // The program will then output 999 if the input value is below 8, output 1000 if
//     // the input value is equal to 8, or output 1001 if the input value is greater than 8.

//     let data = vec![
//         3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
//         1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
//         1105, 1, 46, 98, 99,
//     ];
//     let mut machine = Machine::new(data);
//     machine.run();
// }
