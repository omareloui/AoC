use std::collections::HashMap;

use aoc::*;
use regex::Regex;

type Res = String;

const DAY_NUMBER: DayNumber = 5;

fn challenge_1(input: &String) -> Res {
	let (cargo, commands) = input.split_once("\n\n").unwrap();

	// parse cargo
	let mut map = HashMap::<u8, Vec<char>>::new();
	let mut cargo_lines = cargo.lines().collect::<Vec<&str>>();

	cargo_lines.pop();

	// parse initial state
	let ends_re = Regex::new(r"(?:^\s{3}|\s{3}$)").unwrap();
	let center_re = Regex::new(r"\s{4}").unwrap();

	cargo_lines.iter().for_each(|line| {
		let line = ends_re.replace_all(&line, "[_]").to_string();
		let line = center_re.replace_all(&line, " [_]").to_string();

		// println!("{}", line);

		let dirty_chars = line.split_whitespace();

		for (i, dirty_char) in dirty_chars.enumerate() {
			let key = (i + 1) as u8;
			let char = dirty_char.chars().collect::<Vec<char>>()[1];

			if char == '_' {
				continue;
			}

			match map.get(&(key)) {
				None => {
					map.insert(key, vec![char]);
				}
				Some(value) => {
					let mut new = value.to_owned();
					new.insert(0, char);
					map.insert(key, new);
				}
			};
		}
	});

	// apply commands
	let re = Regex::new(r"move (\d{1,2}) from (\d{1,2}) to (\d{1,2})").unwrap();
	for cap in re.captures_iter(commands) {
		let amount = cap[1].parse::<u8>().unwrap();
		let from_index = cap[2].parse::<u8>().unwrap();
		let to_index = cap[3].parse::<u8>().unwrap();

		for _ in 0..amount {
			let mut from = map.get(&from_index).unwrap().to_owned();
			let mut to = map.get(&to_index).unwrap().to_owned();

			let tmp = from.pop().unwrap();
			to.push(tmp);
			map.insert(from_index, from);
			map.insert(to_index, to);
		}
	}

	let mut final_str: Vec<char> = vec!['_'; map.len()];

	for key in map.keys() {
		let final_char = map.get(key).unwrap().last().unwrap().to_owned();
		final_str[(key - 1) as usize] = final_char;
	}

	return final_str.iter().collect::<String>();
}

fn challenge_2(input: &String) -> Res {
	unimplemented!()
}

fn main() {
	let test_results: (Res, Res) = (String::from("CMZ"), String::from(""));

	let day = Day::new(
		DAY_NUMBER,
		test_results,
		Box::new(challenge_1),
		Box::new(challenge_2),
		true,
	);
	day.check();
}
