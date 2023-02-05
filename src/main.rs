use bibe_asm::parser::{
	parse,
	tokenize,
};

mod emitter;

fn main() {
	let mut emitter = emitter::Emitter::new();
    let (s, tokens) = tokenize("mov r1, 2\nmov r2, r4").unwrap();
	let (s, statements) = parse(&tokens).unwrap();
	let mut file = std::fs::File::create("out.bin").unwrap();
	println!("{:?}", statements);
    emitter.emit(&statements);
	emitter.write(&mut file);
}
