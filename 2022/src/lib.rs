use std::{fs, io::Error, ops::Deref};

pub type DayNumber = u8;
pub type Res = u32;

pub enum PartNumber {
	One,
	Two,
}

pub enum Mode {
	Test,
	Prod,
}

fn build_path(day: DayNumber, mode: &Mode, challenge_number: &PartNumber) -> String {
	let src = format!("./src/bin/day{}", day);

	let mode = match mode {
		Mode::Prod => "prod",
		Mode::Test => "test",
	};
	let num = match challenge_number {
		PartNumber::One => "1",
		PartNumber::Two => "2",
	};

	format!("{}/input_{}.{}", src, num, mode)
}

pub fn get_input_content(
	day: DayNumber,
	mode: &Mode,
	challenge_number: &PartNumber,
) -> Result<String, Error> {
	fs::read_to_string(build_path(day, mode, challenge_number))
}

type PartFunction = Box<dyn Fn(&String) -> Res>;

struct Part<'a> {
	test_result: Res,
	num: &'a PartNumber,
	prod_input: String,
	test_input: String,
	function: PartFunction,
}

impl Part<'static> {
	pub fn new<'a>(
		day: DayNumber,
		num: &'a PartNumber,
		test_result: Res,
		function: PartFunction,
	) -> Part<'a> {
		Part {
			test_result,
			num,
			test_input: get_input_content(day, &Mode::Test, num)
				.expect("Can't find the input test file"),
			prod_input: get_input_content(day, &Mode::Prod, &num)
				.expect("Can't find the input prod file"),
			function,
		}
	}

	// TODO: find a way to get this &u32 from the ResultType
	pub fn run(&self, mode: &Mode) -> Res {
		return match mode {
			// TODO: find a prettier way to write this deref
			Mode::Test => self.function.deref()(&self.test_input),
			Mode::Prod => self.function.deref()(&self.prod_input),
		};
	}

	pub fn check_part(&self) {
		println!("");
		let test_res = self.run(&Mode::Test);
		let prod_res = self.run(&Mode::Prod);
		let mut _num = match &self.num {
			PartNumber::One => 1,
			PartNumber::Two => 2,
		};

		println!("Passed test {}: {:?}", _num, self.test_result == test_res);
		println!("Test result {}: {:?}", _num, test_res);
		println!("Prod result {}: {:?}", _num, prod_res);

		println!("");
	}
}

pub struct Day<'a> {
	part_1: Part<'a>,
	part_2: Part<'a>,
}

impl Day<'static> {
	pub fn new<'a>(
		day: DayNumber,
		results: (Res, Res),
		function_1: PartFunction,
		function_2: PartFunction,
	) -> Day<'a> {
		Day {
			part_1: Part::new(day, &PartNumber::One, results.0, function_1),
			part_2: Part::new(day, &PartNumber::Two, results.1, function_2),
		}
	}

	pub fn check(&self) {
		self.part_1.check_part();
		self.part_2.check_part();
	}
}
