/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_asm::parser::{
	parse,
	tokenize, Statement,
};
use clap::{ Arg, Command };
use std::fs::{
	read_to_string,
	File,
};
use simplelog::{
	Config as LogConfig,
	LevelFilter,
	WriteLogger,
};

mod emitter;

fn main() {
	let matches = Command::new("as")
		.arg(Arg::new("input").required(true))
		.arg(Arg::new("out")
			.short('o')
			.long("output")
			.required(true)
		)
		.arg(Arg::new("format")
			.short('f')
			.long("format")
			.default_value("img")
		)
		.get_matches();

	let log_file = File::create("log.txt").expect("Failed to create log file");
	WriteLogger::init(LevelFilter::Debug, LogConfig::default(), log_file).expect("Failed to init logger");

	let inputs = matches.get_many::<String>("input").unwrap();
	let mut output = File::create(matches.get_one::<String>("out").unwrap()).expect("Failed to open output file");
	let format = matches.get_one::<String>("format").unwrap();
	let format = match &format[..] {
		"ann" => emitter::Kind::Annotated,
		"img" => emitter::Kind::Image,
		_ => panic!("Unsupported output format"),
	};

	let mut origin = 0u32;
	for input in inputs {
		let contents = read_to_string(input).expect("Failed to read file"); 

		let (_, tokens) = tokenize(&contents).unwrap();
		let (_, statements) = parse(&tokens).unwrap();

		for statement in &statements {
			if origin & 0x3 != 0 {
				// Align the address
				origin = origin + 3 & 0xfffffff3;
			}

			match statement {
				Statement::Instruction(instruction) => {
					emitter::emit_instruction(format, &mut output, origin, instruction);
					origin += 4;
				}
			}
		}
	}
}
