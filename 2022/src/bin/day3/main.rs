use aoc::*;

const DAY_NUMBER: DayNumber = 3;
const TEST_RESULTS: (Res, Res) = (157, 70);

fn challenge_1(input: &String) -> Res {
	input
		.lines()
		.map(|line| {
			let (one, two) = line.split_at(line.len() / 2);

			let mut common_char: char = '-';

			for o_ch in one.chars() {
				for t_ch in two.chars() {
					if o_ch == t_ch {
						common_char = o_ch;
						break;
					}
				}
			}

			if common_char == '-' {
				panic!("Can't find the common char.")
			}

			if common_char.is_lowercase() {
				common_char as u32 - 96
			} else {
				common_char as u32 - 38
			}
		})
		.sum()
}

fn challenge_2(input: &String) -> Res {
	input
		.lines()
		.collect::<Vec<&str>>()
		.chunks(3)
		.map(|group_lines| {
			let mut common_char: char = '-';

			for f_ch in group_lines[0].chars() {
				for s_ch in group_lines[1].chars() {
					for t_ch in group_lines[2].chars() {
						if f_ch == s_ch && s_ch == t_ch {
							common_char = f_ch;
							break;
						}
					}
				}
			}

			if common_char == '-' {
				panic!("Can't find the common char.")
			}

			if common_char.is_lowercase() {
				common_char as u32 - 96
			} else {
				common_char as u32 - 38
			}
		})
		.sum()
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
