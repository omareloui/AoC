use aoc::*;

const DAY_NUMBER: DayNumber = 1;
const TEST_RESULTS: (Res, Res) = (24000, 45000);

fn challenge_1(input: &String) -> Res {
	input
		.split("\n\n")
		.map(|bulk| {
			return bulk
				.lines()
				.map(|line| line.parse::<u32>().expect("Can't parse this thing"))
				.sum::<u32>();
		})
		.max()
		.unwrap()
}

fn challenge_2(input: &String) -> Res {
	let mut parsed: Vec<u32> = input
		.split("\n\n")
		.map(|bulk| {
			return bulk
				.lines()
				.map(|line| line.parse::<u32>().expect("Can't parse this thing"))
				.sum::<u32>();
		})
		.collect();
	parsed.sort_by(|a, b| b.cmp(a));
	parsed.iter().take(3).sum()
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
