use aoc::*;

const DAY_NUMBER: DayNumber = 4;
const TEST_RESULTS: (Res, Res) = (2, 4);

fn challenge_1(input: &String) -> Res {
	input
		.lines()
		.filter_map(|line| {
			let ranges = line.split(",").collect::<Vec<&str>>();
			let (range1, range2) = (ranges[0], ranges[1]);

			let split_range1 = range1
				.split("-")
				.map(|minmax| minmax.parse::<u32>().unwrap())
				.collect::<Vec<u32>>();
			let split_range2 = range2
				.split("-")
				.map(|minmax| minmax.parse::<u32>().unwrap())
				.collect::<Vec<u32>>();

			let (min1, max1) = (split_range1[0], split_range1[1]);
			let (min2, max2) = (split_range2[0], split_range2[1]);

			let mut overlap: Vec<Res> = vec![];

			let range_1 = (min1..(max1 + 1)).collect::<Vec<u32>>();
			let range_2 = (min2..(max2 + 1)).collect::<Vec<u32>>();

			for id_1 in &range_1 {
				for id_2 in &range_2 {
					if id_1 == id_2 {
						overlap.push(*id_1);
					}
				}
			}

			if overlap == range_1 || overlap == range_2 {
				Some(1)
			} else {
				None
			}
		})
		.sum()
}

fn challenge_2(input: &String) -> Res {
	input
		.lines()
		.filter_map(|line| {
			let ranges = line.split(",").collect::<Vec<&str>>();
			let (range1, range2) = (ranges[0], ranges[1]);

			let split_range1 = range1
				.split("-")
				.map(|minmax| minmax.parse::<u32>().unwrap())
				.collect::<Vec<u32>>();
			let split_range2 = range2
				.split("-")
				.map(|minmax| minmax.parse::<u32>().unwrap())
				.collect::<Vec<u32>>();

			let (min1, max1) = (split_range1[0], split_range1[1]);
			let (min2, max2) = (split_range2[0], split_range2[1]);

			let mut overlap: Vec<Res> = vec![];

			let range_1 = (min1..(max1 + 1)).collect::<Vec<u32>>();
			let range_2 = (min2..(max2 + 1)).collect::<Vec<u32>>();

			for id_1 in &range_1 {
				for id_2 in &range_2 {
					if id_1 == id_2 {
						overlap.push(*id_1);
					}
				}
			}

			if overlap.len() > 0 {
				Some(1)
			} else {
				None
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
