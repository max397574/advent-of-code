use regex::*;

struct BeaconDistance {
    coords: (i128, i128),
    distance: i128,
}

impl BeaconDistance {
    fn get_covered_fields_on_row(&self, y: i128) -> Vec<i128> {
        let mut covered = Vec::new();
        let y_distance = self.coords.1.max(y) - self.coords.1.min(y);
        if self.distance < y_distance {
            return covered;
        }
        for x in self.coords.0 - (self.distance - y_distance)
            ..=self.coords.0 + (self.distance - y_distance)
        {
            covered.push(x);
        }
        covered
    }
}

fn get_beacon_distances_and_beacons(input: &str) -> (Vec<BeaconDistance>, Vec<(i128, i128)>) {
    let re =
        Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)").unwrap();
    let mut beacon_distances = Vec::with_capacity(input.lines().count());
    let mut beacons = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        if let Some((_full, [sensor_x, sensor_y, beacon_x, beacon_y])) =
            re.captures(line).map(|caps| caps.extract())
        {
            beacons.push((beacon_x.parse().unwrap(), beacon_y.parse().unwrap()));
            beacon_distances.push(BeaconDistance {
                coords: (sensor_x.parse().unwrap(), sensor_y.parse().unwrap()),
                distance: ((sensor_x.parse::<i128>().unwrap() - beacon_x.parse::<i128>().unwrap())
                    .abs()
                    + (sensor_y.parse::<i128>().unwrap() - beacon_y.parse::<i128>().unwrap())
                        .abs()),
            });
        };
    }
    (beacon_distances, beacons)
}

pub fn part_1(_input: &str) -> impl std::fmt::Display {
    let (beacon_distances, mut beacons) = get_beacon_distances_and_beacons(input);
    beacons.sort();
    beacons.dedup();
    let mut covered_fields = Vec::new();
    for sensor in beacon_distances.iter() {
        sensor
            .get_covered_fields_on_row(row_to_check)
            .iter()
            .for_each(|field| covered_fields.push(*field))
    }
    covered_fields.sort_unstable();
    covered_fields.dedup();
    for beacon in beacons.iter() {
        // just remove one element if beacon lays on the line to check and is covered because it's
        // about amount and not specific ones
        if beacon.1 == row_to_check && covered_fields.contains(&beacon.0) {
            covered_fields.pop();
        }
    }
    covered_fields.len()
}

pub fn part_2(_input: &str) -> impl std::fmt::Display {
    let (beacon_distances, mut beacons) = get_beacon_distances_and_beacons(input);
    beacons.sort();
    beacons.dedup();
    let mut covered_fields = Vec::new();
    covered_fields.sort_unstable();
    covered_fields.dedup();
    // for i in
}

// #[cfg(test)]
mod tests {
    use super::*;
    const _INPUT1: &str = "";
    const _INPUT2: &str = "";

    // #[test]
    fn _part1() {
        assert_eq!(part_1(_INPUT1).to_string(), String::from("0"))
    }

    // #[test]
    fn _part2() {
        assert_eq!(part_2(_INPUT2).to_string(), String::from("0"))
    }
}
