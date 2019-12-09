use advent2019::Machine;

fn main() {
    let mut m = Machine::from_file("data/09");
    m.run();
}



#[test]
fn example_9_1a() {
    let data = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let (tx, rx) = std::sync::mpsc::channel();
    let output = move |val| tx.send(val).unwrap();
    {
        let mut machine = Machine::new_with_io(data.clone(), || panic!(), output);
        machine.run();
    }

    let output: Vec<_> = rx.iter().collect();
    assert_eq!(data, output);
}

#[test]
fn example_9_1b() {
    // check compiles and doesn't crash
    let data = vec![1102,34915192,34915192,7,4,7,99,0];
    let mut machine = Machine::new(data);
    machine.run();
}

#[test]
fn example_9_1c() {
    // check compiles and doesn't crash
    let data = vec![104,1125899906842624,99];
    let mut machine = Machine::new(data);
    machine.run();
}
