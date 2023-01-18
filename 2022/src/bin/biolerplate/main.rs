use aoc::*;

const DAY_NUMBER: DayNumber = 0;
const TEST_RESULTS: (Res, Res) = (0, 0);

fn challenge_1(input: &String) -> Res {
	unimplemented!()
}

fn challenge_2(input: &String) -> Res {
	unimplemented!()
}

fn main() {
	let day = Day::new(
		DAY_NUMBER,
		TEST_RESULTS,
		Box::new(challenge_1),
		Box::new(challenge_2),
	);
	day.check();
}
