/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bibe_asm::asm;
use bibe_asm::parser::StringID;

use std::collections::{
	BTreeMap,
	HashMap
};

use log::debug;

const PAGE_LEN: u64 = 4096;

pub struct Page {
	byte_offset: u64,
	pub statements: [Option<(u64, asm::Statement)>; PAGE_LEN as usize]
}

impl Page {
	pub fn new() -> Page {
		Page {
			byte_offset: 0,
			statements: [None; PAGE_LEN as usize]
		}
	}
}

pub struct State {
	addr: u64,
	pub pages: BTreeMap<u64, Page>,
	pub symbols: HashMap<StringID, u64>,
}

impl State {
	pub fn new() -> State {
		State {
			addr: 0,
			pages: BTreeMap::new(),
			symbols: HashMap::new(),
		}
	}

	fn get_page(&mut self, addr: u64) -> (&mut Page, u64) {
		let page_addr = (addr >> 14) << 14;
		if !self.pages.contains_key(&page_addr) {
			self.pages.insert(page_addr, Page::new());
		}

		let page = self.pages.get_mut(&page_addr).unwrap();
		let mut index = (addr - page_addr) / 4;

		while page.statements[index as usize].is_some() {
			index = index + 1;
			if index >= PAGE_LEN {
				//TODO: add proper handling for this
				panic!("Statement insertion overflow");
			}
		}

		(page, index)
	}

	fn align_addr(addr: u64, align: u64) -> u64 {
		let mut i = 0;
		while addr & (1 << i) == 0 { i = i + 1 }
		((addr + align) >> i) << i
	}

	fn insert_symbol(&mut self, id: Option<StringID>, value: u64) {
		if let Some(id) = id {
			self.symbols.insert(id, value);
		}
	}

	pub fn insert_statement(&mut self, statement: &asm::Statement) {
		if statement.is_instruction() {
			// Align the address if required
			if self.addr & 0x3 != 0 {
				self.addr = self.addr + 3 & 0xfffffffffffffff3;
			}
		}

		if let asm::Statement::Directive(directive) = statement {
			use asm::Directive;
			match directive {
				Directive::Origin(origin) => {
					self.addr = *origin
				},
				Directive::Align(align) => self.addr = Self::align_addr(self.addr, *align),
				Directive::Label(id) => self.insert_symbol(Some(*id), self.addr),

				Directive::Byte(id, value) => self.insert_symbol(*id, *value as u64),
				Directive::Short(id, value) => self.insert_symbol(*id, *value as u64),
				Directive::Word(id, value) => self.insert_symbol(*id, *value as u64),
				Directive::Quad(id, value) => self.insert_symbol(*id, *value as u64),
			}
		}

		if statement.size_of() == 0 {
			return
		}

		let (page, index) = self.get_page(self.addr);
		page.statements[index as usize] = Some((page.byte_offset, *statement));
		page.byte_offset += statement.size_of();
		self.addr += statement.size_of();
	}
}