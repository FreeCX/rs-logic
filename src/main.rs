mod element;

use element::*;

// NOT X = X NAND X
fn init_not() -> Block {
    // init hardcoded NAND component
    let nand = Block::init_nand();
    // init user NOT component (composit)
    let mut not = Block::init("NOT", 1, 1);
    // push NAND component into NOT
    let nand_id = not.push_element(nand);
    // connect all wires
    not.connect_wire(Wire::InputSelf(0), Wire::Input(nand_id, 0));
    not.connect_wire(Wire::InputSelf(0), Wire::Input(nand_id, 1));
    not.connect_wire(Wire::Output(nand_id, 0), Wire::OutputSelf(0));
    not
}

//              A     B     C
// X AND Y = (NOT X) NOR (NOT Y)
fn init_and() -> Block {
    let not_a = init_not();
    let not_b = init_not();
    let nor = Block::init_nor();
    // init user AND component
    let mut and = Block::init("AND", 2, 1);
    // push all components
    let nid = and.push_element(nor);
    let aid = and.push_element(not_a);
    let bid = and.push_element(not_b);
    // connect A
    and.connect_wire(Wire::InputSelf(0), Wire::Input(aid, 0));
    // connect C
    and.connect_wire(Wire::InputSelf(1), Wire::Input(bid, 0));
    // connect B
    and.connect_wire(Wire::Output(aid, 0), Wire::Input(nid, 0));
    and.connect_wire(Wire::Output(bid, 0), Wire::Input(nid, 1));
    // result to output
    and.connect_wire(Wire::Output(nid, 0), Wire::OutputSelf(0));
    and
}

//             A     B      C
// X OR Y = (NOT X) NAND (NOT Y)
fn init_or() -> Block {
    let not_a = init_not();
    let not_b = init_not();
    let nand = Block::init_nand();
    // init user OR component
    let mut or = Block::init("OR", 2, 1);
    // push all components
    let nid = or.push_element(nand);
    let aid = or.push_element(not_a);
    let bid = or.push_element(not_b);
    // connect A
    or.connect_wire(Wire::InputSelf(0), Wire::Input(aid, 0));
    // connect C
    or.connect_wire(Wire::InputSelf(1), Wire::Input(bid, 0));
    // connect B
    or.connect_wire(Wire::Output(aid, 0), Wire::Input(nid, 0));
    or.connect_wire(Wire::Output(bid, 0), Wire::Input(nid, 1));
    // result to output
    or.connect_wire(Wire::Output(nid, 0), Wire::OutputSelf(0));
    or
}

fn main() {
    let mut and = init_and();
    println!("AND component:");
    for (w1, w2) in vec![(0, 0), (0, 1), (1, 0), (1, 1)] {
        and.set_input_wire(0, w1);
        and.set_input_wire(1, w2);
        and.execute();
        println!("{:?}", and);
    }
    let mut or = init_or();
    println!("OR component:");
    for (w1, w2) in vec![(0, 0), (0, 1), (1, 0), (1, 1)] {
        or.set_input_wire(0, w1);
        or.set_input_wire(1, w2);
        or.execute();
        println!("{:?}", or);
    }
}