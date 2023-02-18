use std::collections::HashMap;

use aoc::*;

type Res = u32;

const DAY_NUMBER: DayNumber = 7;
const TEST_RESULTS: (Res, Res) = (95437, 24933642);

#[derive(Clone, Debug)]
struct FileInfo {
	is_dir: bool,
	size: u32,
	name: String,
}

type TotalSize = u32;
type SystemMap = HashMap<String, (TotalSize, Vec<FileInfo>)>;

fn challenge_1(input: &String) -> Res {
	let mut g_dir = String::from("/");
	let mut system_map: SystemMap = HashMap::new();

	// parse the file
	input.lines().for_each(|line| {
		if line.starts_with("$") {
			let command = line.split_once(" ").unwrap().1;
			if command.starts_with("cd ") {
				let dir_cmd = command.split_once(" ").unwrap().1;
				g_dir = cd(dir_cmd, &g_dir);
			}
		} else {
			let files_option = system_map.get(&g_dir);
			if let None = files_option {
				system_map.insert(g_dir.to_owned(), (0, vec![]));
			}
			let (total_size, mut files) = system_map.get(&g_dir).unwrap().to_owned();
			let is_dir = line.starts_with("dir");
			let name = line.split_once(" ").unwrap().1.to_string();
			let size: u32 = if is_dir {
				0
			} else {
				line.split_once(" ").unwrap().0.parse::<u32>().unwrap()
			};

			let file_info = FileInfo { is_dir, name, size };
			files.push(file_info);
			system_map.insert(g_dir.to_owned(), ((total_size + size), files));
		}
	});

	fn cd(new_dir: &str, current_dir: &String) -> String {
		match new_dir {
			"/" => String::from("/"),
			".." => {
				let mut dirs = current_dir.split("/").collect::<Vec<_>>();
				dirs.pop();
				let dir = dirs.join("/");

				if dir == "" {
					return String::from("/");
				} else {
					return dir;
				}
			}
			other => {
				let mut new_dir = current_dir.to_owned();
				if new_dir != "/" {
					new_dir.push('/');
				}
				new_dir.push_str(other);
				new_dir
			}
		}
	}

	// calculate sup dir total size
	let new_vec = system_map
		.iter()
		.map(|hash_value| {
			let (mut size, files) = system_map.get(hash_value.0).unwrap();

			for file_info in files {
				if !file_info.is_dir {
					continue;
				}
				size = get_dir_size(&system_map, hash_value.0, &file_info);
			}

			return (hash_value.0, (size, files));
		})
		.collect::<Vec<_>>();

	fn get_dir_size(system_map: &SystemMap, dir: &String, file_info: &FileInfo) -> u32 {
		if !file_info.is_dir {
			panic!("it can't be a file")
		}

		let map_value = system_map.get(dir).unwrap();
		let mut size = map_value.0;

		for file_info in map_value.1.iter() {
			if !file_info.is_dir {
				continue;
			}
			size += get_dir_size(system_map, &cd(&file_info.name, &dir), file_info)
		}

		return size;
	}

	// get all under 100_000
	let sum: u32 = new_vec
		.iter()
		.filter_map(|v| {
			let size = v.1 .0;
			if size < 100_001 {
				return Some(size);
			}
			return None;
		})
		.sum();

	return sum;
}

fn challenge_2(input: &String) -> Res {
	let total_space = 70000000;
	let space_needed = 30000000;
	let mut g_dir = String::from("/");
	let mut system_map: SystemMap = HashMap::new();

	// parse the file
	input.lines().for_each(|line| {
		if line.starts_with("$") {
			let command = line.split_once(" ").unwrap().1;
			if command.starts_with("cd ") {
				let dir_cmd = command.split_once(" ").unwrap().1;
				g_dir = cd(dir_cmd, &g_dir);
			}
		} else {
			let files_option = system_map.get(&g_dir);
			if let None = files_option {
				system_map.insert(g_dir.to_owned(), (0, vec![]));
			}
			let (total_size, mut files) = system_map.get(&g_dir).unwrap().to_owned();
			let is_dir = line.starts_with("dir");
			let name = line.split_once(" ").unwrap().1.to_string();
			let size: u32 = if is_dir {
				0
			} else {
				line.split_once(" ").unwrap().0.parse::<u32>().unwrap()
			};

			let file_info = FileInfo { is_dir, name, size };
			files.push(file_info);
			system_map.insert(g_dir.to_owned(), ((total_size + size), files));
		}
	});

	fn cd(new_dir: &str, current_dir: &String) -> String {
		match new_dir {
			"/" => String::from("/"),
			".." => {
				let mut dirs = current_dir.split("/").collect::<Vec<_>>();
				dirs.pop();
				let dir = dirs.join("/");

				if dir == "" {
					return String::from("/");
				} else {
					return dir;
				}
			}
			other => {
				let mut new_dir = current_dir.to_owned();
				if new_dir != "/" {
					new_dir.push('/');
				}
				new_dir.push_str(other);
				new_dir
			}
		}
	}

	// calculate sup dir total size
	let new_vec = system_map
		.iter()
		.map(|hash_value| {
			let (mut size, files) = system_map.get(hash_value.0).unwrap();

			for file_info in files {
				if !file_info.is_dir {
					continue;
				}
				size = get_dir_size(&system_map, hash_value.0, &file_info);
			}

			return (hash_value.0, (size, files));
		})
		.collect::<Vec<_>>();

	fn get_dir_size(system_map: &SystemMap, dir: &String, file_info: &FileInfo) -> u32 {
		if !file_info.is_dir {
			panic!("it can't be a file")
		}

		let map_value = system_map.get(dir).unwrap();
		let mut size = map_value.0;

		for file_info in map_value.1.iter() {
			if !file_info.is_dir {
				continue;
			}
			size += get_dir_size(system_map, &cd(&file_info.name, &dir), file_info)
		}

		return size;
	}

	let mut sizes = new_vec.iter().map(|x| (x.0, x.1 .0)).collect::<Vec<_>>();
	sizes.sort_by_key(|x| x.1);
	let current_used_space = sizes.iter().find(|x| x.0 == "/").unwrap().1;
	let free_space = total_space - current_used_space;
	let needed_space_to_free = space_needed - free_space;

	let mut candidate_to_delete: Vec<u32> = vec![];

	for info in new_vec {
		let size = info.1 .0;
		if size >= needed_space_to_free {
			candidate_to_delete.push(size);
		}
	}

	candidate_to_delete.sort();

	return candidate_to_delete[0];
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
