use std::collections::HashMap;
use std::time::Instant;

extern crate rayon;
use rayon::prelude::*;

fn get_seeds(seed_line: &str) -> Vec<i64> {
    seed_line
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse().expect("Not a number"))
        .collect()
}

fn get_map_ranges(lines: &Vec<&str>) -> Vec<Vec<Vec<i64>>> {
    let mut map_ranges: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut current_range: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        if line.ends_with("map:") {
            if !current_range.is_empty() {
                map_ranges.push(current_range.clone());
                current_range.clear();
            }
        } else {
            let values: Vec<i64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if !values.is_empty() {
                current_range.push(values);
            }
        }
    }

    if !current_range.is_empty() {
        map_ranges.push(current_range);
    }

    map_ranges
}

fn process_seed(seed: i64, ranges: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut seed = seed;

    for map_ranges in ranges {
        let mut smallest_diff: Option<i64> = None;

        for map_range in map_ranges {
            let dst = map_range[0];
            let src = map_range[1];
            let range = map_range[2];

            if seed >= src && seed < src + range {
                let diff = dst - src;
                smallest_diff = match smallest_diff {
                    Some(current) => Some(current.min(diff)),
                    None => Some(diff),
                };
            }
        }

        if let Some(diff) = smallest_diff {
            seed += diff;
        }
    }

    seed
}

fn main() {
    let start = Instant::now();

    let mut lines = include_str!("../input.txt").lines();

    let _seeds = get_seeds(lines.next().unwrap());

    let ranges = get_map_ranges(&lines.collect());

    let seed_2_loc: HashMap<_, _> = (3_119_409_201..3_362_633_271)
        .into_par_iter()
        .map(|seed| (seed, process_seed(seed, &ranges)))
        .collect();

    let p2 = seed_2_loc.values().min().unwrap();

    assert_eq!(*p2, 77_435_348);

    println!("Part 2: {}", p2);

    println!("{:?}", Instant::now() - start);
}

// ----------------------------
// Sorted seeds
// ----------------------------
// 1848591090 		462385043
// 2611025720 		154883670
// 1508373603 		11536371
// 3692308424 		16905163
// 1203540561 		280364121
// 3755585679 		337861951
// 93589727 		738327409
// 3421539474 		257441906
// 3119409201 		243224070
// 50985980 		7961058
// ----------------------------
// Ranges
// ----------------------------
// 50_985_980		58_947_038
// 93_589_727		831_917_136
// 1_203_540_561	1_483_904_682
// 1_508_373_603	1_519_909_974
// 1_848_591_090	2_310_976_133
// 2_611_025_720	2_765_909_390
// 3_119_409_201	3_362_633_271
// 3_421_539_474	3_678_981_380
// 3_692_308_424	3_709_213_587
// 3_755_585_679	4_093_447_630
// ----------------------------
// 1526666011
// 292211340
// 562057898
// 3520380446
// 95461669
// 77435348 <--------- Solution
// 243583567
// 791316226
// 653925960
// ----------------------------
