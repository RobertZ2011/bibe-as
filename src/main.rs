use bibe_asm::parser::{
	parse,
	tokenize,
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
	let mut emitter = emitter::Emitter::new();
	let matches = Command::new("as")
		.arg(Arg::new("input").required(true))
		.arg(Arg::new("out")
			.short('o')
			.long("output")
			.required(true)
		)
		.get_matches();

	let log_file = File::create("log.txt").expect("Failed to create log file");
	WriteLogger::init(LevelFilter::Debug, LogConfig::default(), log_file).expect("Failed to init logger");

	let inputs = matches.get_many::<String>("input").unwrap();
	let mut output = File::create(matches.get_one::<String>("out").unwrap()).expect("Failed to open output file");

	for input in inputs {
		let contents = read_to_string(input).expect("Failed to read file"); 

		let (_, tokens) = tokenize(&contents).unwrap();
		let (_, statements) = parse(&tokens).unwrap();
		emitter.emit(&statements);
	}

	emitter.write(&mut output);
}
