mod element;

use element::*;

fn main() {
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
    for input in 0..2 {
        // set value for input wire
        not.set_input_wire(0, input);
        // calculating
        not.execute();
        println!("NOT: {:?}", not);
    }
}