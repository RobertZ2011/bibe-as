/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_asm::parser::{
	parse,
	tokenize, string_table,
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

use bibe_asm::asm::{
	emitter,
	object,
};

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
			.default_value("bin")
		)
		.get_matches();

	let log_file = File::create("log.txt").expect("Failed to create log file");
	WriteLogger::init(LevelFilter::Debug, LogConfig::default(), log_file).expect("Failed to init logger");

	let inputs = matches.get_many::<String>("input").unwrap();
	let output = File::create(matches.get_one::<String>("out").unwrap()).expect("Failed to open output file");
	let format = matches.get_one::<String>("format").unwrap();
	let mut e = emitter::create(&format, Box::new(output)).unwrap();

	for input in inputs {
		let contents = read_to_string(input).expect("Failed to read file"); 

		let (_, tokens) = tokenize(&contents).unwrap();
		let (tokens, statements) = parse(&tokens).unwrap();

		let mut object = object::Object::new();
		for statement in &statements {
			log::debug!("{statement:?}");
			object.insert_statement(statement);
		}

		let res = e.emit(&object);
		if let Err(err) = res{
			log::debug!("{:?}", tokens);
			match err {
				emitter::Error::UndefinedSymbol(id) => println!("Undefined symbol {}", string_table::lookup(id).unwrap()),
			}
		}
	}
}
