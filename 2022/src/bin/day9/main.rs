use aoc::*;

type Res = u32;

const DAY_NUMBER: DayNumber = 9;
const TEST_RESULTS: (Res, Res) = (13, 36);

#[derive(Debug)]
enum Direction {
	Right,
	Left,
	Up,
	Down,
}

fn count_hashes(grid: &Vec<Vec<&str>>) -> Res {
	return grid
		.into_iter()
		.flatten()
		.filter_map(|x| match x {
			&"#" => Some(1),
			_ => None,
		})
		.sum();
}

fn parse_commands(input: &String) -> Vec<(Direction, usize)> {
	return input
		.lines()
		.map(|line| line.split_once(" ").unwrap())
		.map(|(direction, count)| {
			let mut dir_from_enum = Direction::Right;
			if direction == "U" {
				dir_from_enum = Direction::Up
			} else if direction == "L" {
				dir_from_enum = Direction::Left
			} else if direction == "D" {
				dir_from_enum = Direction::Down
			}
			(dir_from_enum, count.parse::<usize>().unwrap())
		})
		.collect::<Vec<(Direction, usize)>>();
}

fn challenge_1(input: &String) -> Res {
	// let grid_size = 6;
	let grid_size = 360;

	let mut grid = vec![vec!["."; grid_size]; grid_size];

	// let initial_pos = (grid_size - 1, 0);
	let initial_pos = (grid_size / 2, grid_size / 2);

	let mut head_pos = initial_pos;
	let mut tail_pos = initial_pos;

	grid[head_pos.0][head_pos.1] = "#";

	let commands = parse_commands(input);

	fn update_head(grid: &mut Vec<Vec<&str>>, head_position: (usize, usize)) {
		grid[head_position.0][head_position.1] = "H";
	}

	fn update_tail(grid: &mut Vec<Vec<&str>>, tail_position: (usize, usize)) {
		grid[tail_position.0][tail_position.1] = "#";
	}

	fn is_same_pos(head_pos: &(usize, usize), tail_pos: &(usize, usize)) -> bool {
		return head_pos.0 == tail_pos.0 && head_pos.1 == tail_pos.1;
	}

	fn is_head_one_unit_apart_from_tail(
		head_pos: &(usize, usize),
		tail_pos: &(usize, usize),
	) -> bool {
		let on_same_row = head_pos.0 == tail_pos.0;
		let on_same_col = head_pos.1 == tail_pos.1;

		let x_diff = head_pos.0 as isize - tail_pos.0 as isize;
		let y_diff = head_pos.1 as isize - tail_pos.1 as isize;

		let apart_on_x_axis = x_diff == 1 || x_diff == -1;
		let apart_on_y_axis = y_diff == 1 || y_diff == -1;

		return (on_same_row && apart_on_y_axis) || (on_same_col && apart_on_x_axis);
	}

	fn is_head_two_units_apart_from_tail(
		head_pos: &(usize, usize),
		tail_pos: &(usize, usize),
	) -> bool {
		let on_same_row = head_pos.0 == tail_pos.0;
		let on_same_col = head_pos.1 == tail_pos.1;

		let x_diff = head_pos.0 as isize - tail_pos.0 as isize;
		let y_diff = head_pos.1 as isize - tail_pos.1 as isize;

		let apart_on_x_axis = x_diff == 2 || x_diff == -2;
		let apart_on_y_axis = y_diff == 2 || y_diff == -2;

		let is_diagnal_apart = ((x_diff == 2 || x_diff == -2) && (y_diff == 1 || y_diff == -1))
			|| ((y_diff == 2 || y_diff == -2) && (x_diff == 1 || x_diff == -1));

		return (on_same_row && apart_on_y_axis)
			|| (on_same_col && apart_on_x_axis)
			|| is_diagnal_apart;
	}

	fn is_head_diagnal_to_tail(head_pos: &(usize, usize), tail_pos: &(usize, usize)) -> bool {
		let x_diff = head_pos.0 as isize - tail_pos.0 as isize;
		let y_diff = head_pos.1 as isize - tail_pos.1 as isize;
		return (x_diff == 1 || x_diff == -1) && (y_diff == 1 || y_diff == -1);
	}

	commands.into_iter().for_each(|(direction, count)| {
		// ..##..
		// ...##.
		// .####.
		// ....#.
		// ####..

		// println!("======= {count:?} {direction:?} =======");

		for _ in 0..count {
			match direction {
				Direction::Right => {
					head_pos.1 += 1;
				}
				Direction::Left => {
					head_pos.1 -= 1;
				}
				Direction::Up => {
					head_pos.0 -= 1;
				}
				Direction::Down => {
					head_pos.0 += 1;
				}
			};
			// update_head(&mut grid, head_pos);

			if !is_head_two_units_apart_from_tail(&head_pos, &tail_pos) {
				continue;
			}

			// if is_head_one_unit_apart_from_tail(&head_pos, &tail_pos) {
			// 	continue;
			// }

			// if is_head_diagnal_to_tail(&head_pos, &tail_pos) {
			// 	println!("========== is dignal ==========");
			// 	grid.clone().into_iter().for_each(|row| {
			// 		println!("{row:?}");
			// 	});
			// 	println!("========== end dignal ==========");
			// 	// tail_pos = head_pos;
			// 	continue;
			// }

			match direction {
				Direction::Right => {
					tail_pos.0 = head_pos.0;
					tail_pos.1 = head_pos.1 - 1;
				}
				Direction::Left => {
					tail_pos.0 = head_pos.0;
					tail_pos.1 = head_pos.1 + 1;
				}
				Direction::Up => {
					tail_pos.1 = head_pos.1;
					tail_pos.0 = head_pos.0 + 1;
				}
				Direction::Down => {
					tail_pos.1 = head_pos.1;
					tail_pos.0 = head_pos.0 - 1;
				}
			};

			// match direction {
			// 	Direction::Right => {
			// 		tail_pos.1 = head_pos.1 - 1;
			// 	}
			// 	Direction::Left => {
			// 		tail_pos.1 = head_pos.1 + 1;
			// 	}
			// 	Direction::Up => {
			// 		tail_pos.0 = head_pos.0 + 1;
			// 	}
			// 	Direction::Down => {
			// 		tail_pos.0 = head_pos.0 - 1;
			// 	}
			// };

			// if is_same_pos(&head_pos, &tail_pos) {
			// 	tail_pos = head_pos;
			// 	update_tail(&mut grid, tail_pos);
			// 	continue;
			// };

			// // if !is_head_one_unit_apart_from_tail(&head_pos, &tail_pos) {
			// // 	tail_pos = head_pos;
			// // 	continue;
			// // }

			update_tail(&mut grid, tail_pos);
		}

		// grid.clone().into_iter().for_each(|row| {
		// 	println!("{row:?}");
		// });
	});

	// println!("");
	// println!("========= FINAL =========");
	// grid.clone().into_iter().for_each(|row| {
	// 	println!("{row:?}");
	// });

	let hashes_count = count_hashes(&grid);

	// println!("{hashes_count:?}");
	// println!("========= END =========");
	// println!("");
	// println!("");

	return hashes_count;
}

fn challenge_2(input: &String) -> Res {
	let grid_size = (32, 32);
	// let grid_size = 360;

	let mut grid = vec![vec!["."; grid_size.0]; grid_size.1];

	let initial_pos = (grid_size.1 / 2, grid_size.0 / 2);

	let mut head_pos = initial_pos;
	let mut tail_pos = initial_pos;

	// grid[head_pos.0][head_pos.1] = "#";
	grid[head_pos.0][head_pos.1] = "S";

	let commands = parse_commands(input);

	fn update_head(grid: &mut Vec<Vec<&str>>, head_position: (usize, usize)) {
		grid[head_position.0][head_position.1] = "H";
	}

	fn update_tail(grid: &mut Vec<Vec<&str>>, tail_position: (usize, usize)) {
		grid[tail_position.0][tail_position.1] = "#";
	}

	fn is_head_nine_units_apart_from_tail(
		head_pos: &(usize, usize),
		tail_pos: &(usize, usize),
	) -> bool {
		let on_same_row = head_pos.0 == tail_pos.0;
		let on_same_col = head_pos.1 == tail_pos.1;

		let x_diff = head_pos.0 as isize - tail_pos.0 as isize;
		let y_diff = head_pos.1 as isize - tail_pos.1 as isize;

		let apart_on_x_axis = x_diff == 9 || x_diff == -9;
		let apart_on_y_axis = y_diff == 9 || y_diff == -9;

		let is_diagnal_apart = ((x_diff == 9 || x_diff == -9) && (y_diff == 1 || y_diff == -1))
			|| ((y_diff == 9 || y_diff == -9) && (x_diff == 1 || x_diff == -1));

		return (on_same_row && apart_on_y_axis)
			|| (on_same_col && apart_on_x_axis)
			|| is_diagnal_apart;
	}

	fn is_head_diagnal_to_tail(head_pos: &(usize, usize), tail_pos: &(usize, usize)) -> bool {
		let x_diff = head_pos.0 as isize - tail_pos.0 as isize;
		let y_diff = head_pos.1 as isize - tail_pos.1 as isize;
		return (x_diff == 1 || x_diff == -1) && (y_diff == 1 || y_diff == -1);
	}

	commands.into_iter().for_each(|(direction, count)| {
		println!("======= {count:?} {direction:?} =======");

		for _ in 0..count {
			match direction {
				Direction::Right => {
					head_pos.1 += 1;
				}
				Direction::Left => {
					head_pos.1 -= 1;
				}
				Direction::Up => {
					head_pos.0 -= 1;
				}
				Direction::Down => {
					head_pos.0 += 1;
				}
			};
			update_head(&mut grid, head_pos);

			if !is_head_nine_units_apart_from_tail(&head_pos, &tail_pos) {
				continue;
			}

			match direction {
				Direction::Right => {
					tail_pos.1 = head_pos.1 - 9;
				}
				Direction::Left => {
					tail_pos.1 = head_pos.1 + 9;
				}
				Direction::Up => {
					tail_pos.0 = head_pos.0 + 9;
				}
				Direction::Down => {
					tail_pos.0 = head_pos.0 - 9;
				}
			};

			update_tail(&mut grid, tail_pos);
		}

		grid.clone().into_iter().for_each(|row| {
			println!("{}", row.join(" "));
		});
	});

	println!("");
	println!("========= FINAL =========");
	grid.clone().into_iter().for_each(|row| {
		println!("{}", row.join(" "));
	});

	let hashes_count = count_hashes(&grid);

	println!("Hashes count: {hashes_count:?}.");
	println!("========= END =========");
	println!("");
	println!("");

	panic!("end");

	return hashes_count;
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
