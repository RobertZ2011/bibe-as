use bibe_asm::parser::{
	parse,
	tokenize,
};

fn main() {
    let (s, tokens) = tokenize("add r1, r0, 2\nadd r2, r0, 3").unwrap();
	let (s, statements) = parse(&tokens).unwrap();
    println!("{:?} {:?}", statements, s);
}
