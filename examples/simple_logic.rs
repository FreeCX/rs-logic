extern crate rs_logic;

use rs_logic::element::*;

// NOT X = X NAND X
pub fn init_not() -> Element {
    // init user NOT component (composit)
    let mut not = Element::init("NOT", 1, 1);
    // push NAND component into NOT
    let nand_id = not.push_element(Element::init_nand());
    // connect all wires
    not.connect_wire(Wire::InputSelf(0), Wire::Input(nand_id, 0));
    not.connect_wire(Wire::InputSelf(0), Wire::Input(nand_id, 1));
    not.connect_wire(Wire::Output(nand_id, 0), Wire::OutputSelf(0));
    not
}

//            A     B
// X OR Y = NOT (X NAND Y)
pub fn init_and() -> Element {
    // init user AND component
    let mut and = Element::init("AND", 2, 1);
    // push all components
    let a = and.push_element(init_not());
    let b = and.push_element(Element::init_nand());
    // B = X NAND Y
    and.connect_wire(Wire::InputSelf(0), Wire::Input(b, 0));
    and.connect_wire(Wire::InputSelf(1), Wire::Input(b, 1));
    // A = NOT B
    and.connect_wire(Wire::Output(b, 0), Wire::Input(a, 0));
    // set result
    and.connect_wire(Wire::Output(a, 0), Wire::OutputSelf(0));
    and
}

//            A     B
// X OR Y = NOT (X NOR Y)
pub fn init_or() -> Element {
    // init user OR component
    let mut or = Element::init("OR", 2, 1);
    // push all components
    let a = or.push_element(init_not());
    let b = or.push_element(Element::init_nor());
    // B = X NOR Y
    or.connect_wire(Wire::InputSelf(0), Wire::Input(b, 0));
    or.connect_wire(Wire::InputSelf(1), Wire::Input(b, 1));
    // A = NOT B
    or.connect_wire(Wire::Output(b, 0), Wire::Input(a, 0));
    // set result
    or.connect_wire(Wire::Output(a, 0), Wire::OutputSelf(0));
    or
}

//               A       B     C
// X XOR Y = (X NAND Y) AND (X OR Y)
pub fn init_xor() -> Element {
    // init user XOR component
    let mut xor = Element::init("XOR", 2, 1);
    // push all components
    let a = xor.push_element(Element::init_nand());
    let b = xor.push_element(init_and());
    let c = xor.push_element(init_or());
    // connect A
    xor.connect_wire(Wire::InputSelf(0), Wire::Input(a, 0));
    xor.connect_wire(Wire::InputSelf(1), Wire::Input(a, 1));
    // connect C
    xor.connect_wire(Wire::InputSelf(0), Wire::Input(c, 0));
    xor.connect_wire(Wire::InputSelf(1), Wire::Input(c, 1));
    // connect B
    xor.connect_wire(Wire::Output(a, 0), Wire::Input(b, 0));
    xor.connect_wire(Wire::Output(c, 0), Wire::Input(b, 1));
    // result to output
    xor.connect_wire(Wire::Output(b, 0), Wire::OutputSelf(0));
    xor
}

fn main() {
    let mut not = init_not();
    println!("NOT component:");
    for v1 in vec![0, 1] {
        not.set_input_wire(0, v1);
        not.execute();
        println!("  [{}] -> {:?}", v1, not.get_output());
    }
    let mut and = init_and();
    let mut and_result = String::new();
    let mut or = init_or();
    let mut or_result = String::new();
    let mut xor = init_xor();
    let mut xor_result = String::new();
    for (v1, v2) in vec![(0, 0), (0, 1), (1, 0), (1, 1)] {
        and.set_input(vec![v1, v2]);
        and.execute();
        or.set_input(vec![v1, v2]);
        or.execute();
        xor.set_input(vec![v1, v2]);
        xor.execute();
        and_result.push_str(&format!("\n  {:?} -> {:?}", &[v1, v2], and.get_output()));
        or_result.push_str(&format!("\n  {:?} -> {:?}", &[v1, v2], or.get_output()));
        xor_result.push_str(&format!("\n  {:?} -> {:?}", &[v1, v2], xor.get_output()));
    }
    println!("AND component:{}", and_result);
    println!("OR component:{}", or_result);
    println!("XOR component:{}", xor_result);
}