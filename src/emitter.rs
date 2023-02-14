/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_asm::parser::{
	Statement,
	StatementStream,
};
use bibe_instr::{
	Encode,
	Instruction,
};
use std::io;

use log::debug;

pub struct Emitter {
	addr: usize,
	image: Vec<u8>,
}

impl Emitter {
	pub fn new() -> Emitter {
		Emitter {
			addr: 0,
			image: vec![0; 1024],
		}
	}

	fn emit_instruction(&mut self, instr: &Instruction) {
		let encoded = instr.encode();
		debug!("Encoded {:?} as {:08x}", instr, encoded);
		if self.addr & 0x3 != 0 {
			// pad the address
			self.addr = self.addr + 3 & 0xfffffff3;
		}

		self.image[self.addr] = (encoded & 0xff) as u8;
		self.image[self.addr + 1] = ((encoded >> 8) & 0xff) as u8;
		self.image[self.addr + 2] = ((encoded >> 16) & 0xff) as u8;
		self.image[self.addr + 3] = ((encoded >> 24) & 0xff) as u8;

		self.addr += 4;
	}

	pub fn emit(&mut self, statements: &StatementStream) {
		for statement in statements {
			match statement {
				Statement::Instruction(i) => self.emit_instruction(i),
				_ => unreachable!()
			}
		}
	}

	pub fn write(&self, w: &mut dyn io::Write) -> io::Result<usize> {
		w.write(&self.image)
	}
}