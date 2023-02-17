use aoc::*;

type Res = u32;

const DAY_NUMBER: DayNumber = 2;
const TEST_RESULTS: (Res, Res) = (15, 12);

fn challenge_1(input: &String) -> Res {
	input
		.lines()
		.map(|line| {
			let _res: Vec<&str> = line.split_whitespace().take(2).collect();
			let (elf, me) = (_res[0], _res[1]);
			let mut score: u32 = 0;

			if matches!(me, "X") {
				if matches!(elf, "A") {
					score += 3 + 1
				} else if matches!(elf, "B") {
					score += 0 + 1
				} else if matches!(elf, "C") {
					score += 6 + 1
				}
			} else if matches!(me, "Y") {
				if matches!(elf, "A") {
					score += 6 + 2
				} else if matches!(elf, "B") {
					score += 3 + 2
				} else if matches!(elf, "C") {
					score += 0 + 2
				}
			} else if matches!(me, "Z") {
				if matches!(elf, "A") {
					score += 0 + 3
				} else if matches!(elf, "B") {
					score += 6 + 3
				} else if matches!(elf, "C") {
					score += 3 + 3
				}
			}

			score
		})
		.sum::<Res>()
}

fn challenge_2(input: &String) -> Res {
	input
		.lines()
		.map(|line| {
			let _res: Vec<&str> = line.split_whitespace().take(2).collect();
			let (elf, me) = (_res[0], _res[1]);
			let mut score: u32 = 0;

			if me == "Y" {
				score += 3
			} else if me == "Z" {
				score += 6
			}

			// A -> Rock => x: scissors (3), y: rock (1), z: paper (2)
			if elf == "A" {
				if me == "X" {
					score += 3
				} else if me == "Y" {
					score += 1
				} else if me == "Z" {
					score += 2
				}
			}
			// B -> Paper => x: rock (1), y: paper (2), z: scissors (3)
			else if elf == "B" {
				if me == "X" {
					score += 1
				} else if me == "Y" {
					score += 2
				} else if me == "Z" {
					score += 3
				}
			}
			// C -> Scissors => x: paper (2), y: scissors (3), z: rock (1)
			else if elf == "C" {
				if me == "X" {
					score += 2
				} else if me == "Y" {
					score += 3
				} else if me == "Z" {
					score += 1
				}
			}

			score
		})
		.sum::<Res>()
}

fn main() {
	let day = Day::new(
		DAY_NUMBER,
		TEST_RESULTS,
		Box::new(challenge_1),
		Box::new(challenge_2),
		false,
	);
	day.check();
}
