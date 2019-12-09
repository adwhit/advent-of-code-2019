use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use itertools::Itertools;

use advent2019::Machine;

fn main() {
    let machine = Machine::from_file("data/07");
    let best = find_best_amplitude(machine.state(), false);
    println!("{}", best);
    let best = find_best_amplitude(machine.state(), true);
    println!("{}", best);
}

fn run_chained_machines(data: &[i64], phases: (i64, i64, i64, i64, i64)) -> i64 {
    let (tx1, rx1) = sync_channel(0);
    let (tx2, rx2) = sync_channel(0);
    let (tx3, rx3) = sync_channel(0);
    let (tx4, rx4) = sync_channel(0);
    let (tx5, rx5) = sync_channel(0);
    let (tx6, rx6) = sync_channel(0);

    let machines = vec![
        Machine::new_with_io(data.to_vec(), takes_input(phases.0, rx1), send_output(tx2)),
        Machine::new_with_io(data.to_vec(), takes_input(phases.1, rx2), send_output(tx3)),
        Machine::new_with_io(data.to_vec(), takes_input(phases.2, rx3), send_output(tx4)),
        Machine::new_with_io(data.to_vec(), takes_input(phases.3, rx4), send_output(tx5)),
        Machine::new_with_io(data.to_vec(), takes_input(phases.4, rx5), send_output(tx6)),
    ];

    let _handles: Vec<_> = machines
        .into_iter()
        .map(|mut m| thread::spawn(move || m.run()))
        .collect();
    tx1.send(0).unwrap();
    loop {
        let out = rx6.recv().unwrap();
        if let Err(_) = tx1.send(out) {
            return out;
        }
    }
}

fn find_best_amplitude(data: &[i64], loopback: bool) -> i64 {
    let range = if loopback { 5..=9 } else { 0..=4 };
    range
        .permutations(5)
        .map(|combo| run_chained_machines(data, (combo[0], combo[1], combo[2], combo[3], combo[4])))
        .max()
        .unwrap()
}

fn takes_input(inp: i64, rx: Receiver<i64>) -> impl FnMut() -> i64 {
    let mut has_fired = false;
    move || {
        if has_fired {
            rx.recv().unwrap()
        } else {
            has_fired = true;
            inp
        }
    }
}

fn send_output(tx: SyncSender<i64>) -> impl FnMut(i64) {
    move |val| tx.send(val).unwrap()
}

#[test]
fn example7_1a() {
    let data = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let out = find_best_amplitude(&data, false);
    assert_eq!(out, 43210);
}

#[test]
fn example7_1b() {
    let data = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    let out = find_best_amplitude(&data, false);
    assert_eq!(out, 54321);
}

#[test]
fn example7_1c() {
    let data = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let out = find_best_amplitude(&data, false);
    assert_eq!(out, 65210);
}

#[test]
fn example7_2a() {
    let data = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let out = find_best_amplitude(&data, true);
    assert_eq!(out, 139629729);
}

#[test]
fn example7_2b() {
    let data = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    let out = find_best_amplitude(&data, true);
    assert_eq!(out, 18216);
}
