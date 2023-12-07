use crate::utils::main;

fn get_contents(
    filename: &str,
) -> (
    Vec<i64>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
    Vec<(i64, i64, i64)>,
) {
    let lines = crate::utils::read_lines(filename);
    let mut toks = lines[0].split(": ");
    let _ = toks.next().unwrap().trim();
    let seeds = toks
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let parse_line = |line: &String| {
        let mut toks = line.split(" ");
        let dst = toks.next().unwrap().trim().parse::<i64>().unwrap();
        let src = toks.next().unwrap().trim().parse::<i64>().unwrap();
        let len = toks.next().unwrap().trim().parse::<i64>().unwrap();
        return (dst, src, len);
    };

    let mut seed_to_soil_map = Vec::new();
    let mut i = 3;
    while lines[i] != "" {
        seed_to_soil_map.push(parse_line(&lines[i]));
        i += 1;
    }

    let mut soil_to_fertilizer_map = Vec::new();
    i += 2;
    while lines[i] != "" {
        soil_to_fertilizer_map.push(parse_line(&lines[i]));
        i += 1;
    }

    let mut fertilizer_to_water_map = Vec::new();
    i += 2;
    while lines[i] != "" {
        fertilizer_to_water_map.push(parse_line(&lines[i]));
        i += 1;
    }

    let mut water_to_light_map = Vec::new();
    i += 2;
    while lines[i] != "" {
        water_to_light_map.push(parse_line(&lines[i]));
        i += 1;
    }

    let mut light_to_temperature_map = Vec::new();
    i += 2;
    while lines[i] != "" {
        light_to_temperature_map.push(parse_line(&lines[i]));
        i += 1;
    }

    let mut temperature_to_humidity_map = Vec::new();
    i += 2;
    while lines[i] != "" {
        temperature_to_humidity_map.push(parse_line(&lines[i]));
        i += 1;
    }

    let mut humidity_to_location_map = Vec::new();
    i += 2;
    while i < lines.len() {
        humidity_to_location_map.push(parse_line(&lines[i]));
        i += 1;
    }

    return (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    );
}

fn apply(map: &Vec<(i64, i64, i64)>, val: i64) -> i64 {
    for (dst, src, len) in map {
        if val >= *src && val < *src + *len {
            let pos = val - *src;
            return *dst + pos;
        }
    }
    return val;
}

fn part1(
    input: &(
        Vec<i64>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
    ),
) -> i64 {
    let (
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ) = input;
    let soils = seeds.iter().map(|seed| apply(seed_to_soil_map, *seed));
    let fertilizers = soils.map(|soil| apply(soil_to_fertilizer_map, soil));
    let waters = fertilizers.map(|fertilizer| apply(fertilizer_to_water_map, fertilizer));
    let lights = waters.map(|water| apply(water_to_light_map, water));
    let temperatures = lights.map(|light| apply(light_to_temperature_map, light));
    let humidities = temperatures.map(|temp| apply(temperature_to_humidity_map, temp));
    return humidities
        .map(|humidity| apply(humidity_to_location_map, humidity))
        .min()
        .unwrap();
}

fn part2(
    input: &(
        Vec<i64>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
        Vec<(i64, i64, i64)>,
    ),
) -> i64 {
    let (
        seed_ranges,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temperature_map,
        temperature_to_humidity_map,
        humidity_to_location_map,
    ) = input;
    let soils = seed_ranges
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .flatten()
        .map(|seed| apply(seed_to_soil_map, seed));
    let fertilizers = soils.map(|soil| apply(soil_to_fertilizer_map, soil));
    let waters = fertilizers.map(|fertilizer| apply(fertilizer_to_water_map, fertilizer));
    let lights = waters.map(|water| apply(water_to_light_map, water));
    let temperatures = lights.map(|light| apply(light_to_temperature_map, light));
    let humidities = temperatures.map(|temp| apply(temperature_to_humidity_map, temp));
    return humidities
        .map(|humidity| apply(humidity_to_location_map, humidity))
        .min()
        .unwrap();
}

main!(5);
