#![allow(dead_code)]
extern crate rs_logic;

mod simple_logic;

use rs_logic::element::*;
use simple_logic::{init_or, init_not};

// X OR-NOT Y = NOT (X OR Y)
fn init_or_not() -> Element {
    let mut or_not = Element::init("OR-NOT", 2, 1);
    let or = or_not.push_element(init_or());
    let not = or_not.push_element(init_not());
    or_not.connect_wire(Wire::InputSelf(0), Wire::Input(or, 0));
    or_not.connect_wire(Wire::InputSelf(1), Wire::Input(or, 1));
    or_not.connect_wire(Wire::Output(or, 0), Wire::Input(not, 0));
    or_not.connect_wire(Wire::Output(not, 0), Wire::OutputSelf(0));
    or_not
}

fn init_rs_trigger() -> Element {
    let mut rs = Element::init("RS", 2, 2);
    rs.set_initial_data(vec![0, 0], vec![0, 1]);
    let o1 = rs.push_element(init_or_not());
    let o2 = rs.push_element(init_or_not());
    // RS
    rs.connect_wire(Wire::InputSelf(0), Wire::Input(o1, 0));
    rs.connect_wire(Wire::InputSelf(1), Wire::Input(o2, 1));
    // feedback
    rs.connect_wire(Wire::Output(o1, 0), Wire::Input(o2, 0));
    rs.connect_wire(Wire::Output(o2, 0), Wire::Input(o1, 1));
    // Q and NOT Q
    rs.connect_wire(Wire::Output(o1, 0), Wire::OutputSelf(0));
    rs.connect_wire(Wire::Output(o2, 0), Wire::OutputSelf(1));
    rs
}

fn main() {
    // RS initial output is (0, 0)
    let mut rs = init_rs_trigger();
    let input_data = vec![(1, 0), (1, 1), (0, 1)];
    let output_data = vec![(0, 1), (0, 0), (1, 0)];
    println!("RS trigger\n R  S       Q nQ | Q nQ");
    //      R   S     Q  nQ
    for (&(v1, v2), (r1, r2)) in input_data.iter().zip(output_data) {
        let state = (rs.get_output_wire(0), rs.get_output_wire(1));
        rs.set_input(vec![v1, v2]);
        rs.execute();
        let r = rs.get_output();
        println!("{:?} --> {:?} {:?}", &[v1, v2], state, r);
    }
}