use std::{cmp::PartialEq, fs, io::Error, ops::Deref};

pub type DayNumber = u8;

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

type PartFunction<T> = Box<dyn Fn(&String) -> T>;

struct Part<'a, T> {
	test_result: T,
	num: &'a PartNumber,
	prod_input: String,
	test_input: String,
	function: PartFunction<T>,
}

impl<T> Part<'static, T>
where
	T: PartialEq + std::fmt::Debug,
{
	pub fn new<'a>(
		day: DayNumber,
		num: &'a PartNumber,
		test_result: T,
		function: PartFunction<T>,
		take_input_num: Option<&PartNumber>,
	) -> Part<'a, T> {
		let input_num: &PartNumber = match take_input_num {
			Some(value) => value,
			None => num,
		};

		Part {
			test_result,
			num,
			test_input: get_input_content(day, &Mode::Test, input_num)
				.expect("Can't find the input test file"),
			prod_input: get_input_content(day, &Mode::Prod, input_num)
				.expect("Can't find the input prod file"),
			function,
		}
	}

	pub fn run(&self, mode: &Mode) -> T {
		return match mode {
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

pub struct Day<'a, T> {
	part_1: Part<'a, T>,
	part_2: Part<'a, T>,
}

impl<T> Day<'static, T>
where
	T: PartialEq + std::fmt::Debug,
{
	pub fn new<'a>(
		day: DayNumber,
		results: (T, T),
		function_1: PartFunction<T>,
		function_2: PartFunction<T>,
		same_input: bool,
	) -> Day<'a, T> {
		Day {
			part_1: Part::new(day, &PartNumber::One, results.0, function_1, None),
			part_2: Part::new(
				day,
				&PartNumber::Two,
				results.1,
				function_2,
				if same_input {
					Some(&PartNumber::One)
				} else {
					None
				},
			),
		}
	}

	pub fn check(&self) {
		self.part_1.check_part();
		self.part_2.check_part();
	}
}
