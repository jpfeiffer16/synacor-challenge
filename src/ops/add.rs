use ops::operation::Operation;
pub struct Add;

impl Operation for Add {
	fn len(&self) -> usize {
		4
	}
	fn is_jump(&self) -> bool {
		false
	}
	fn run(&self, _: u16, _: &Vec<u16>, _: &mut Vec<u16>, _: &mut Vec<u16>) -> usize {
		0
	}
}