use aoc::*;

type Res = u32;

const DAY_NUMBER: DayNumber = 8;
const TEST_RESULTS: (Res, Res) = (21, 8);

fn challenge_1(input: &String) -> Res {
	let grid = input
		.lines()
		.map(|x| x.chars().collect::<Vec<char>>())
		.map(|x| {
			x.iter()
				.map(|y| y.to_string().parse::<u8>().unwrap())
				.collect::<Vec<u8>>()
		})
		.collect::<Vec<Vec<u8>>>();

	let rows_count = grid.len();
	let columns_count = grid[0].len();
	let mut candidates: u32 = ((rows_count * 2) + (columns_count * 2) - 4) as u32;

	for (i, r) in grid.iter().enumerate() {
		for (j, _) in r.iter().enumerate() {
			if j == 0 || j == rows_count - 1 || i == 0 || i == columns_count - 1 {
				continue;
			}

			let current = grid[i][j];

			// top
			let mut is_visiable = true;
			for top_index in 0..i {
				let top = grid[top_index][j];
				if top >= current {
					is_visiable = false;
					break;
				}
			}
			if is_visiable == true {
				candidates += 1;
				continue;
			}

			// bottom
			is_visiable = true;
			for bottom_index in (i + 1)..rows_count {
				let bottom = grid[bottom_index][j];
				if bottom >= current {
					is_visiable = false;
					break;
				}
			}
			if is_visiable == true {
				candidates += 1;
				continue;
			}

			// left
			is_visiable = true;
			for left_index in 0..j {
				let left = grid[i][left_index];
				if left >= current {
					is_visiable = false;
					break;
				}
			}
			if is_visiable == true {
				candidates += 1;
				continue;
			}

			// right
			is_visiable = true;
			for right_index in (j + 1)..columns_count {
				let right = grid[i][right_index];
				if right >= current {
					is_visiable = false;
					break;
				}
			}
			if is_visiable == true {
				candidates += 1;
				continue;
			}
		}
	}

	return candidates;
}

fn challenge_2(input: &String) -> Res {
	let grid = input
		.lines()
		.map(|x| x.chars().collect::<Vec<char>>())
		.map(|x| {
			x.iter()
				.map(|y| y.to_string().parse::<u8>().unwrap())
				.collect::<Vec<u8>>()
		})
		.collect::<Vec<Vec<u8>>>();

	let rows_count = grid.len();
	let columns_count = grid[0].len();
	let mut scores: Vec<u32> = vec![];

	for (i, r) in grid.iter().enumerate() {
		for (j, _) in r.iter().enumerate() {
			if j == 0 || j == rows_count - 1 || i == 0 || i == columns_count - 1 {
				continue;
			}

			let mut score = vec![0; 4];

			let current = grid[i][j];

			// top
			for top_index in (0..i).rev() {
				let top = grid[top_index][j];
				score[0] += 1;
				if top >= current {
					break;
				}
			}

			// bottom
			for bottom_index in (i + 1)..rows_count {
				let bottom = grid[bottom_index][j];
				score[1] += 1;
				if bottom >= current {
					break;
				}
			}

			// left
			for left_index in (0..j).rev() {
				let left = grid[i][left_index];
				score[2] += 1;
				if left >= current {
					break;
				}
			}

			// right
			for right_index in (j + 1)..columns_count {
				let right = grid[i][right_index];
				score[3] += 1;
				if right >= current {
					break;
				}
			}

			let score = score.iter().fold(1, |acc, x| acc * x);
			scores.push(score);
		}
	}

	scores.sort();
	return scores[scores.len() - 1];
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
