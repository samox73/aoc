#[cfg(test)]
mod tests {
    use crate::day05::{
        find_lowest_location_with_ranged_seeds, get_location_from_seed,
        get_lowest_maps_range_length, parse, parse_range_map,
    };

    #[test]
    fn parse_range_map_works() {
        let input = "humidity-to-location map:
59 56 37
55 93 4
";
        let result = parse_range_map(input);
        assert!(result.is_ok());
    }

    #[test]
    fn map_seeds() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let almanac = parse(input).unwrap().1;
        let solution = find_lowest_location_with_ranged_seeds(almanac);
        println!("solution: {}", solution);
    }
}
