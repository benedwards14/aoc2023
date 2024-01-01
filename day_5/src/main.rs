use std::{fs::read_to_string, str::Lines};


#[derive(Clone)]
struct Section {
    start_idx: i64,
    end_idx: i64
}

struct MapEntry {
    source_start: i64,
    source_end: i64,
    destination_start: i64
}

struct Map {
    entries: Vec<MapEntry>
}

struct Mappings {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

fn parse_mappings(lines: &mut Lines) -> Map {
    let mut entries = Vec::new();

    for line in lines {
        match line {
            "" => break,
            a => {
                entries.push(MapEntry::parse(a));
            }

        }
    }

    entries.sort_by_key(|k| k.source_start);
    Map { entries }
}

fn parse_seed_line(line: &str) -> Vec<i64> {
    let mut seeds: Vec<i64> = Vec::new();

    for seed in line[7..].split_whitespace() {
        seeds.push(seed.parse().unwrap());
    }

    seeds
}

fn parse_seeds_1(line: &str) -> Vec<Section> {
    let parsed = parse_seed_line(line);
    let mut seeds: Vec<Section> = Vec::new();

    for id in parsed {
        seeds.push(Section{ start_idx: id, end_idx: id });
    }

    seeds
}

fn parse_seeds_2(line: &str) -> Vec<Section> {
    let input = parse_seed_line(line);
    let mut seeds: Vec<Section> = Vec::new();

    let section = input.len() / 2;

    for idx in 0..section {
        let start = input[2* idx];
        let range = input[(2*idx) + 1];

        seeds.push(Section{ start_idx: start, end_idx: start + range - 1 })
    }

    println!("{}", seeds[0].start_idx);

    seeds
}

fn parse_input() -> (Vec<Section>, Vec<Section>, Mappings) {
    let binding = read_to_string("./data.txt").unwrap();
    let mut lines =  binding.lines();

    let seed_line = lines.next().unwrap();
    assert!(lines.next().unwrap().is_empty());

    assert!(lines.next().unwrap() == "seed-to-soil map:");
    let seed_to_soil = parse_mappings(&mut lines);

    assert!(lines.next().unwrap() == "soil-to-fertilizer map:");
    let soil_to_fertilizer = parse_mappings(&mut lines);

    assert!(lines.next().unwrap() == "fertilizer-to-water map:");
    let fertilizer_to_water = parse_mappings(&mut lines);

    assert!(lines.next().unwrap() == "water-to-light map:");
    let water_to_light = parse_mappings(&mut lines);

    assert!(lines.next().unwrap() == "light-to-temperature map:");
    let light_to_temperature = parse_mappings(&mut lines);

    assert!(lines.next().unwrap() == "temperature-to-humidity map:");
    let temperature_to_humidity = parse_mappings(&mut lines);

    assert!(lines.next().unwrap() == "humidity-to-location map:");
    let humidity_to_location = parse_mappings(&mut lines);

    (parse_seeds_1(seed_line), parse_seeds_2(seed_line), Mappings {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location
    })
}

impl MapEntry {
    fn parse(line: &str) -> Self {
        let destination_start: i64;
        let source_start: i64;
        let length: i64;
        match line.split_whitespace().collect::<Vec<_>>()[..] {
            [a, b, c] => {
                destination_start = a.parse().unwrap();
                source_start = b.parse().unwrap();
                length = c.parse().unwrap();
            },
            _ => panic!("Oh No!")
        }

        MapEntry {
            source_start, source_end: source_start + length - 1, destination_start
        }
    }

    fn get_overlapping_section(&self, original: &Section) -> Option<Section> {
        if self.source_end < original.start_idx {
            None
        } else if self.source_end <= original.end_idx {
            if self.source_start < original.start_idx {
                Some (Section { start_idx: original.start_idx, end_idx: self.source_end })
            } else {
                Some (Section { start_idx: self.source_start, end_idx: self.source_end })
            }
        } else {
            if self.source_start < original.start_idx {
                Some (Section { start_idx: original.start_idx, end_idx: original.end_idx })
            } else if self.source_start <= original.end_idx {
                Some(Section { start_idx: self.source_start, end_idx: original.end_idx })
            } else {
                None
            }              
        }
    }

    fn map(&self, id: i64) -> i64 {
        id - self.source_start + self.destination_start
    }
}

impl Map {
    fn map(&self, section: &Section, _check:bool) -> Vec<Section> {
        let mut new_sections = Vec::new();

        let mut last_seen_idx: i64 = 0;

        let mut seen_section = false;

        for entry in &self.entries {
            let overlapping_section = entry.get_overlapping_section(section);

            match overlapping_section {
                None if !seen_section  => {},
                None if seen_section => { break; },
                Some(overlap) => {
                    new_sections.push(Section { start_idx: entry.map(overlap.start_idx), end_idx: entry.map(overlap.end_idx) });

                    if overlap.start_idx > section.start_idx && overlap.start_idx != last_seen_idx + 1 {
                        let start = if seen_section {
                            last_seen_idx + 1
                        } else {
                            section.start_idx
                        };
                        new_sections.push(Section { start_idx: start, end_idx: overlap.start_idx - 1 });
                    }
                    seen_section = true;
                }
                _ => panic!("Oh No!")
            }

            last_seen_idx = entry.source_end;
        }

        if !seen_section {
            new_sections.push(section.clone());
        } else if last_seen_idx < section.end_idx {
            new_sections.push(Section { start_idx: last_seen_idx + 1, end_idx: section.end_idx });
        }

        assert!(new_sections.len() > 0);
        // if check {
        //     assert!(new_sections.iter().filter(|a| a.start_idx == 1190847573).collect::<Vec<_>>().len() == 0);
        // }
        new_sections
    }
}

impl Mappings {
    fn get_closest_location(&self, seeds: &Vec<Section>) -> i64 {
        seeds.iter()
        .flat_map(|s| self.seed_to_soil.map(s, true))
        .flat_map(|s| self.soil_to_fertilizer.map(&s, true))
        .flat_map(|s| self.fertilizer_to_water.map(&s, true))
        .flat_map(|s| self.water_to_light.map(&s, true))
        .flat_map(|s| self.light_to_temperature.map(&s, true))
        .flat_map(|s| self.temperature_to_humidity.map(&s, false))
        .flat_map(|s| self.humidity_to_location.map(&s, false))
        .map(|s| s.start_idx)
        .min().unwrap()
    }
}

fn main() {
    let (input1, input2, mappings) = parse_input();

    let location1 = mappings.get_closest_location(&input1);

    println!("{}", location1); // 309796150

    let location2 = mappings.get_closest_location(&input2);

    println!("{}", location2);
}
