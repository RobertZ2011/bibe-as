/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_instr::Instruction;
use std::fs::File;

mod annotated;
mod image;
//pub mod hex;

#[derive(Copy, Clone)]
pub enum Kind {
	Annotated,
	Image,
}

pub fn emit_instruction(kind: Kind, file: &mut File, addr: u32, instruction: &Instruction) {
	match kind {
		Kind::Annotated => annotated::emit_instruction(file, addr, instruction),
		Kind::Image => image::emit_instruction(file, addr, instruction),
	}
}