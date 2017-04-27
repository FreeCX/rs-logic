#![allow(dead_code)]
extern crate rs_logic;

mod simple_logic;

use rs_logic::element::*;
use simple_logic::{init_xor, init_and, init_or};

// | X | Y || S | C |
// +---+---++---+---+
// | 0 | 0 || 0 | 0 |
// | 0 | 1 || 1 | 0 |
// | 1 | 0 || 1 | 0 |
// | 1 | 1 || 0 | 1 |
fn half_adder() -> Element {
    let mut hadd = Element::init("HADD", 2, 2);
    let xid = hadd.push_element(init_xor());
    let aid = hadd.push_element(init_and());
    // S = X XOR Y
    hadd.connect_wire(Wire::InputSelf(0), Wire::Input(xid, 0));
    hadd.connect_wire(Wire::InputSelf(1), Wire::Input(xid, 1));
    hadd.connect_wire(Wire::Output(xid, 0), Wire::OutputSelf(0));
    // C = X AND Y
    hadd.connect_wire(Wire::InputSelf(0), Wire::Input(aid, 0));
    hadd.connect_wire(Wire::InputSelf(1), Wire::Input(aid, 1));
    hadd.connect_wire(Wire::Output(aid, 0), Wire::OutputSelf(1));
    hadd
}

// | A | B | P_i || S | P_{i+1} |
// +---+---+-----++---+---------+
// | 0 | 0 |  0  || 0 |    0    |
// | 1 | 0 |  0  || 1 |    0    |
// | 0 | 1 |  0  || 1 |    0    |
// | 1 | 1 |  0  || 0 |    1    |
// | 0 | 0 |  1  || 1 |    0    |
// | 1 | 0 |  1  || 0 |    1    |
// | 0 | 1 |  1  || 0 |    1    |
// | 1 | 1 |  1  || 1 |    1    |
fn full_adder() -> Element {
    let mut add = Element::init("ADD", 3, 2);
    let h1 = add.push_element(half_adder());
    let h2 = add.push_element(half_adder());
    let or = add.push_element(init_or());
    add.connect_wire(Wire::InputSelf(0), Wire::Input(h1, 0));
    add.connect_wire(Wire::InputSelf(1), Wire::Input(h1, 1));
    add.connect_wire(Wire::InputSelf(2), Wire::Input(h2, 1));
    add.connect_wire(Wire::Output(h1, 0), Wire::Input(h2, 0));
    add.connect_wire(Wire::Output(h1, 1), Wire::Input(or, 0));
    add.connect_wire(Wire::Output(h2, 1), Wire::Input(or, 1));
    add.connect_wire(Wire::Output(h2, 0), Wire::OutputSelf(0));
    add.connect_wire(Wire::Output(or, 0), Wire::OutputSelf(1));
    add
}

fn main() {
    let input_data = vec![
        (0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0),
        (0, 0, 1), (1, 0, 1), (0, 1, 1), (1, 1, 1)
    ];
    let output_data = vec![
        (0, 0), (1, 0), (1, 0), (0, 1),
        (1, 0), (0, 1), (0, 1), (1, 1)
    ];
    let mut add = full_adder();
    println!("Adder:");
    for (&(i1, i2, i3), (o1, o2)) in input_data.iter().zip(output_data) {
        add.set_input(vec![i1, i2, i3]);
        add.execute();
        let r = add.get_output();
        let status = if r[0] == o1 && r[1] == o2 {
            "PASSED"
        } else {
            "FAILED"
        };
        println!("{:?} --> {:?}: {}", &[i1, i2, i3], r, status);
    }
}