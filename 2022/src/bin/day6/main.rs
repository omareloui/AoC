use std::collections::HashSet;

use aoc::*;

type Res = u32;

const DAY_NUMBER: DayNumber = 6;
const TEST_RESULTS: (Res, Res) = (7, 19);

fn challenge_1(input: &String) -> Res {
	let binding = input.chars().collect::<Vec<_>>();
	let windows = binding.windows(4);

	for (index, window) in windows.enumerate() {
		let mut set: HashSet<char> = HashSet::new();

		for char in window {
			set.insert(char.to_owned());
		}

		if set.len() == 4 {
			return (index + 4) as u32;
		}
	}

	panic!("didn't find the marker")
}

fn challenge_2(input: &String) -> Res {
	let binding = input.chars().collect::<Vec<_>>();
	let windows = binding.windows(14);

	for (index, window) in windows.enumerate() {
		let mut set: HashSet<char> = HashSet::new();

		for char in window {
			set.insert(char.to_owned());
		}

		if set.len() == 14 {
			return (index + 14) as u32;
		}
	}

	panic!("didn't find the marker")
}

fn main() {
	let day = Day::new(
		DAY_NUMBER,
		TEST_RESULTS,
		Box::new(challenge_1),
		Box::new(challenge_2),
		true,
	);
	day.check();
}
