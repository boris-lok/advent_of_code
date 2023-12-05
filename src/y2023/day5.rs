use itertools::Itertools;

#[derive(Debug)]
pub struct Mapping {
    map: Vec<(u64, u64, u64)>,
}

impl Mapping {
    fn new() -> Self {
        Self { map: vec![] }
    }

    fn add_mapping(&mut self, p: (u64, u64, u64)) {
        self.map.push(p);
    }

    fn get(&self, target: u64) -> u64 {
        let mut ans = target;
        for (d, s, c) in self.map.iter() {
            if target >= *s && target < (*s + *c) {
                ans = d + (target - s)
            }
        }

        ans
    }
}
#[derive(Debug)]
pub struct Data {
    pub seeds: Vec<u64>,
    seed_to_soil: Mapping,
    soil_to_fertilizer: Mapping,
    fertilizer_to_water: Mapping,
    water_to_light: Mapping,
    light_to_temperature: Mapping,
    temperature_to_humidity: Mapping,
    humidity_to_location: Mapping,
}

impl Data {
    fn seed_to_soil(&self, seeds: &[u64]) -> u64 {
        let soils = seeds
            .iter()
            .map(|e| self.seed_to_soil.get(*e))
            .unique()
            .collect::<Vec<_>>();

        let fertilizers = soils
            .iter()
            .map(|e| self.soil_to_fertilizer.get(*e))
            .unique()
            .collect::<Vec<_>>();

        let waters = fertilizers
            .iter()
            .map(|e| self.fertilizer_to_water.get(*e))
            .unique()
            .collect::<Vec<_>>();

        let lights = waters
            .iter()
            .map(|e| self.water_to_light.get(*e))
            .unique()
            .collect::<Vec<_>>();

        let temperatures = lights
            .iter()
            .map(|e| self.light_to_temperature.get(*e))
            .unique()
            .collect::<Vec<_>>();

        let humidity = temperatures
            .iter()
            .map(|e| self.temperature_to_humidity.get(*e))
            .unique()
            .collect::<Vec<_>>();

        let locations = humidity
            .iter()
            .map(|e| self.humidity_to_location.get(*e))
            .unique()
            .collect::<Vec<_>>();

        locations.into_iter().min().unwrap()
    }
}

pub fn parser(input: &str) -> Data {
    let mut seeds = vec![];
    let mut seed_to_soil = Mapping::new();
    let mut soil_to_fertilizer = Mapping::new();
    let mut fertilizer_to_water = Mapping::new();
    let mut water_to_light = Mapping::new();
    let mut light_to_temperature = Mapping::new();
    let mut temperature_to_humidity = Mapping::new();
    let mut humidity_to_location = Mapping::new();

    let mut cursor = "";

    for l in input.lines() {
        if l.is_empty() {
            continue;
        } else {
            if l.starts_with("seeds: ") {
                let (_, s) = l.split_once("seeds: ").unwrap();
                let ss = s
                    .split(" ")
                    .map(|e| e.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                seeds = ss;
            } else {
                let row = l.split(" ").collect::<Vec<_>>();
                if row.len() == 3 {
                    match cursor {
                        "seed-to-soil" => seed_to_soil.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        "soil-to-fertilizer" => soil_to_fertilizer.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        "fertilizer-to-water" => fertilizer_to_water.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        "water-to-light" => water_to_light.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        "light-to-temperature" => light_to_temperature.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        "temperature-to-humidity" => temperature_to_humidity.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        "humidity-to-location" => humidity_to_location.add_mapping((
                            row[0].parse().unwrap(),
                            row[1].parse().unwrap(),
                            row[2].parse().unwrap(),
                        )),
                        _ => unreachable!(),
                    }
                } else {
                    cursor = row.first().unwrap().trim();
                }
            }
        }
    }

    Data {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use itertools::Itertools;
    use crate::y2023::day5::parser;

    #[test]
    fn parser_works() {
        let input = std::fs::read_to_string("./src/in2023/5.in").unwrap();

        let data = parser(&input);

        // part a
        // dbg!(&data.seed_to_soil(&data.seeds));

        // part b
        let mut seeds = HashSet::new();

        for (b, c) in data.seeds.iter().tuples() {
            let mut i = 0;
            while i < *c {
                seeds.insert(b + i);
                i += 1;
            }
        }

        let s = Vec::from_iter(seeds.into_iter());
        dbg!(&s.len());


        // dbg!(&data.seed_to_soil(&s));
    }
}
