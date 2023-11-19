use std::fs::File;
use std::io::Write;

use bibe_instr as isa;
use bibe_asm::asm as asm;
use isa::Encode;
use crate::state::State;
use crate::emitter::{
	Emitter,
	Result
};

use log::debug;

pub struct Annotated(File);

impl Emitter for Annotated {
	fn emit_isa_instruction(&mut self, _state: &State, addr: u64, instr: &isa::Instruction) -> Result<()> {
		let encoded = instr.encode();
		debug!("Encoded {:?} as {:08x}", instr, encoded);

		let a = (encoded & 0xff) as u8;
		let b = ((encoded >> 8) & 0xff) as u8;
		let c = ((encoded >> 16) & 0xff) as u8;
		let d = ((encoded >> 24) & 0xff) as u8;

		write!(self.0, "{:08x} {:02x} {:02x} {:02x} {:02x} {:?}\n", addr, a, b, c, d, instr).unwrap();
		Ok(())
	}

	fn emit_asm_directive(&mut self, _state: &State, addr: u64, directive: &asm::Directive) -> Result<()> {
		write!(self.0, "{:08x} {:?}\n", addr, directive).unwrap();
		Ok(())
	}
}

pub fn create(file: File) -> Option<Box<dyn Emitter>> {
	Some(Box::new(Annotated(file)))
}