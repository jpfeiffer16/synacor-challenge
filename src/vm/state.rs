pub struct VMState {
	current_instruction: u16,
	memory: Vec<u16>,
	registers: Vec<u16>,
	stack: Vec<u16>,
	console_output: String,
}

impl VMState {
	pub fn new(mem: Vec<u16>) -> VMState {
		VMState {
			current_instruction: 0,
			memory: mem.clone(),
			registers: vec![0, 0, 0, 0, 0, 0, 0, 0],
			stack: Vec::new(),
			console_output: String::new(),
		}
	}

	pub fn get_current_instruction(&self) -> u16 {
		self.current_instruction
	}

	pub fn set_current_instruction(&mut self, value: u16) {
		self.current_instruction = value;
	}

	pub fn get_mem_raw(&self, index: u16) -> u16 {
		self.memory[index as usize]
	}

	pub fn get_mem_segment(&mut self, start: u16, length: u16) -> Vec<u16> {
		self.memory.clone().into_iter().skip(start as usize).take(length as usize).collect::<Vec<u16>>()
	}

	pub fn get_mem_or_register_value(&self, index: u16) -> u16 {
		let memory_value = self.memory[index as usize];
		if memory_value >= 32768 {
			return self.registers[memory_value as usize - 32768];
		}
		memory_value
	}

	pub fn set_memory(&mut self, index: u16, val: u16) {
		if let Some(m) = self.memory.get_mut(index as usize) {
			*m = val;
		}
	}

	pub fn is_valid_memory_address(&self, location: u16) -> bool {
		location as usize <= self.memory.len()
	}

	pub fn set_register(&mut self, register_raw: u16, value: u16) {
		let register = register_raw - 32768;
		if let Some(r) = self.registers.get_mut(register as usize) {
			// println!("Setting register {:?} to {:?}", register, value);
			*r = value;
		}
	}

	pub fn push_stack(&mut self, value: u16) {
		self.stack.push(value);
	}

	pub fn pop_stack(&mut self) -> Option<u16> {
		self.stack.pop()
	}

	pub fn add_to_console_output(&mut self, ch: char) {
		self.console_output.push(ch);
	}

	pub fn get_console_output(&self) -> &str {
		self.console_output.as_str()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_mem_or_register_value_mem() {
		let expected = 1234;
		let registers = vec![0, 0, 0, 0, 0, 0, 0, 0];
		let result = get_mem_or_register_value(expected, &registers);
		assert_eq!(expected, result);
	}

	#[test]
	fn get_mem_or_register_value_reg_0() {
		let expected = 1234;
		let registers = vec![expected, 0, 0, 0, 0, 0, 0, 0];
		let result = get_mem_or_register_value(32768, &registers);
		assert_eq!(expected, result);
	}

	#[test]
	fn get_mem_or_register_value_reg_8() {
		let expected = 1234;
		let registers = vec![0, 0, 0, 0, 0, 0, 0, expected];
		let result = get_mem_or_register_value(32775, &registers);
		assert_eq!(expected, result);
	}
}