#![allow(dead_code)]
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum ElementType {
    Nand,
    Nor,
    Composite
}

#[derive(Debug, Clone, Copy)]
pub enum Wire {
    //        wire
    InputSelf(usize),
    //         wire
    OutputSelf(usize),
    //    id     wire
    Input(usize, usize),
    //     id     wire
    Output(usize, usize)
}

pub trait Element {
    fn init<S: Into<String>>(name: S, in_wire: usize, out_wire: usize) -> Self;

    // work with IO
    fn set_input<T: Into<Vec<u8>>>(&mut self, input: T);
    fn set_input_wire(&mut self, wire: usize, input: u8);
    fn get_output(&self) -> &[u8];
    fn get_output_wire(&self, wire: usize) -> u8;

    fn execute(&mut self);

    // working with inner components
    fn push_element(&mut self, block: Self) -> usize;
    fn connect_wire(&mut self, from: Wire, to: Wire);
}

pub struct Block {
    name: String,
    kind: ElementType,
    input: Vec<u8>,
    output: Vec<u8>,
    components: Vec<Block>,
    connections: Vec<(Wire, Wire)>
}

impl Block {
    fn atomic_init<S: Into<String>>(name: S, kind: ElementType, in_wire: usize, out_wire: usize) -> Block {
        Block {
            name: name.into(),
            kind: kind,
            input: vec![0; in_wire],
            output: vec![0; out_wire],
            components: Vec::new(),
            connections: Vec::new(),
        }
    }
    pub fn init_nand() -> Block {
        Block::atomic_init("NAND", ElementType::Nand, 2, 1)
    }
    pub fn init_nor() -> Block {
        Block::atomic_init("NOR", ElementType::Nor, 2, 1)
    }
    // TODO: check bounds
    fn get_input_wire(&self, wire: usize) -> u8 {
        self.input[wire]
    }
}

impl Element for Block {
    fn init<S: Into<String>>(name: S, in_wire: usize, out_wire: usize) -> Block {
        Block {
            name: name.into(),
            kind: ElementType::Composite,
            input: vec![0; in_wire],
            output: vec![0; out_wire],
            components: Vec::new(),
            connections: Vec::new(),
        }
    }
    fn set_input<T: Into<Vec<u8>>>(&mut self, input: T) {
        self.input = input.into();
    }
    fn set_input_wire(&mut self, wire: usize, input: u8) {
        match self.input.get_mut(wire) {
            Some(block) => *block = input,
            None => panic!("Wire {} for {} out of index!", wire, self.name),
        }
    }
    fn get_output(&self) -> &[u8] {
        self.output.as_slice()
    }
    // TODO: check bounds
    fn get_output_wire(&self, wire: usize) -> u8 {
        self.output[wire]
    }
    fn execute(&mut self) {
        match self.kind {
            ElementType::Nand => self.output[0] = (!self.input[0]) % 2 | (!self.input[1]) % 2,
            ElementType::Nor => self.output[0] = (!self.input[0]) % 2 & (!self.input[1]) % 2,
            ElementType::Composite => {
                for &(from, to) in &self.connections {
                    let input_value = match from {
                        // TODO: write a special function for this
                        Wire::InputSelf(wire) => self.input[wire],
                        // TODO: write a special function for this
                        Wire::Output(id, wire) => self.components[id].get_output_wire(wire),
                        op => panic!("Operation {:?} is not implemented!", op),
                    };
                    match to {
                        // TODO: check bounds or write special function
                        Wire::Input(id, wire) => {
                            self.components[id].set_input_wire(wire, input_value);
                            // TODO: execute if all inputs are set
                            self.components[id].execute();
                        }
                        // TODO: write a special function for this
                        Wire::OutputSelf(wire) => self.output[wire] = input_value,
                        op => panic!("Operation {:?} is not implemented!", op),
                    }
                }
            }
        }
    }
    fn push_element(&mut self, block: Self) -> usize {
        self.components.push(block);
        self.components.len() - 1
    }
    fn connect_wire(&mut self, from: Wire, to: Wire) {
        self.connections.push((from, to));
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block {{ in: {:?}, out: {:?} }}", self.input, self.output)
    }
}