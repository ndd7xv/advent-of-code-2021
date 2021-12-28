use std::{
    cmp,
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
enum Direction {
    X(bool),
    Y(bool),
    Z(bool),
}

#[derive(Debug)]
struct Transformation {
    x: Direction,
    y: Direction,
    z: Direction,
}

impl Transformation {
    fn reverse(&mut self) {
        match self.x {
            Direction::X(bool) => self.x = Direction::X(!bool),
            Direction::Y(bool) => self.x = Direction::Y(!bool),
            Direction::Z(bool) => self.x = Direction::Z(!bool),
        }
        match self.y {
            Direction::X(bool) => self.y = Direction::X(!bool),
            Direction::Y(bool) => self.y = Direction::Y(!bool),
            Direction::Z(bool) => self.y = Direction::Z(!bool),
        }
        match self.z {
            Direction::X(bool) => self.z = Direction::X(!bool),
            Direction::Y(bool) => self.z = Direction::Y(!bool),
            Direction::Z(bool) => self.z = Direction::Z(!bool),
        }
    }
}

#[derive(Debug)]
struct Edge {
    magnitude: f64,
    endpoints: (Beacon, Beacon),
}

struct Scanner {
    beacons: Vec<Beacon>,
    position: Option<(i32, i32, i32)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    fn from(i: impl Iterator<Item = i32>) -> Self {
        let coords = i.collect::<Vec<i32>>();
        Beacon {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}
fn main() {
    let mut scanners = parse_input("inputs/input");

    println!("BEACON COUNT: {}", find_beacon_count(&mut scanners));

    let mut largest_manhattan = get_manhattan_distance(&scanners[0], &scanners[1]);

    for i in 0..scanners.len() - 1 {
        for j in i + 1..scanners.len() {
            let contender = get_manhattan_distance(&scanners[i], &scanners[j]);
            largest_manhattan = cmp::max(contender, largest_manhattan);
        }
    }

    println!("LARGEST MANHATTAN DISTANCE: {}", largest_manhattan)
}

fn get_manhattan_distance(a: &Scanner, b: &Scanner) -> i32 {
    let (ax, ay, az) = a.position.unwrap();
    let (bx, by, bz) = b.position.unwrap();
    (ax - bx).abs() + (ay - by).abs() + (az - bz).abs()
}

fn find_beacon_count(scanners: &mut Vec<Scanner>) -> usize {
    // First, find all of the relative scanner positions
    // Have a list of known scanners to calibrate off of, and loop through the list to constantly check similar edges for each scanner
    // Once a scanner has been 'identified' as sharing at least 12 beacons, modify the scanner to have new coordinates with respect to the known scanner
    let mut unknown_scanners = (1..scanners.len()).collect::<Vec<usize>>();
    let mut known_scanners = vec![0];
    let mut scanner_edges = vec![];

    scanners[0].position = Some((0, 0, 0));

    for scanner in scanners.iter() {
        // Calculate all edges from each beacon
        let mut edges = vec![];
        for i in 0..scanner.beacons.len() - 1 {
            for j in i + 1..scanner.beacons.len() {
                edges.push(Edge {
                    magnitude: calculate_edge_magnitude(&scanner.beacons[i], &scanner.beacons[j]),
                    endpoints: (scanner.beacons[i], scanner.beacons[j]),
                });
            }
        }
        scanner_edges.push(edges);
    }

    while !unknown_scanners.is_empty() {
        'unknown: for unknown_scanner in unknown_scanners.iter() {
            for known_scanner in 0..known_scanners.len() {
                let known_scanner = known_scanners[known_scanner];
                let unknown_shared_magnitudes = scanner_edges[*unknown_scanner]
                    .iter()
                    .filter(|&edge| {
                        scanner_edges[known_scanner]
                            .iter()
                            .any(|other_edge| other_edge.magnitude == edge.magnitude)
                    })
                    .collect::<Vec<&Edge>>();

                // Originally it was 12 but I pinpointed this as the reason I was getting an error - 12 shared magnitudes do not necessarily mean 12 distinct points
                // For example, if 4 points were within the same range, the len() unknown_shared_magnitudes would be 6
                // Technically I could truly check for this, but I'm going off a heuristic thinking that mathematically there is some singular number that suffices
                // The number doesn't change once it's over 17 though (have checked up to 50)
                if unknown_shared_magnitudes.len() > 17 {
                    let known_shared_magnitudes = scanner_edges[known_scanner]
                        .iter()
                        .filter(|&edge| {
                            scanner_edges[*unknown_scanner]
                                .iter()
                                .any(|other_edge| other_edge.magnitude == edge.magnitude)
                        })
                        .collect::<Vec<&Edge>>();

                    let mut direction = calibrate_scanner_position(
                        scanners,
                        *unknown_scanner,
                        unknown_shared_magnitudes,
                        known_shared_magnitudes,
                    );

                    direction.reverse();

                    let mut updated_beacons = vec![];
                    let (x, y, z) = scanners[*unknown_scanner].position.unwrap();
                    for beacon in &scanners[*unknown_scanner].beacons {
                        let (x, y, z) = transform(*beacon, Beacon { x, y, z }, &direction);
                        updated_beacons.push(Beacon { x, y, z });
                    }
                    scanners[*unknown_scanner].beacons = updated_beacons;

                    let scanner = &scanners[*unknown_scanner];
                    let mut edges = vec![];
                    for i in 0..scanner.beacons.len() - 1 {
                        for j in i + 1..scanner.beacons.len() {
                            edges.push(Edge {
                                magnitude: calculate_edge_magnitude(
                                    &scanner.beacons[i],
                                    &scanner.beacons[j],
                                ),
                                endpoints: (scanner.beacons[i], scanner.beacons[j]),
                            });
                        }
                    }
                    scanner_edges[*unknown_scanner] = edges;
                    known_scanners.push(*unknown_scanner);
                    continue 'unknown;
                }
            }
        }
        unknown_scanners.retain(|f| !known_scanners.contains(f));
    }

    let mut beacons = HashSet::new();
    for scanner in scanners {
        for beacon in &scanner.beacons {
            beacons.insert(*beacon);
        }
    }

    beacons.len()
}

fn calibrate_scanner_position(
    scanners: &mut Vec<Scanner>,
    unknown_scanner: usize,
    unknown_shared_magnitudes: Vec<&Edge>,
    known_shared_magnitudes: Vec<&Edge>,
) -> Transformation {
    let mut unknown = &mut scanners[unknown_scanner];

    // Compare the unknown magnitude of one line to the corresponding known magnitude of another
    // From there, identify which beacon coordinate maps to which and then update the scanner with a new position
    let t = *unknown_shared_magnitudes
        .iter()
        .find(|&&edge| edge.magnitude == known_shared_magnitudes[0].magnitude)
        .unwrap();

    let first_point = known_shared_magnitudes[0].endpoints.0;
    let second_point = known_shared_magnitudes[0].endpoints.1;
    let known_edges_with_first_point = known_shared_magnitudes
        .iter()
        .filter(|&&edge| edge.endpoints.0 == first_point || edge.endpoints.1 == first_point);

    let mut first_point_guess = t.endpoints.0;
    let mut second_point_guess = t.endpoints.1;
    let unknown_edges_with_guess = unknown_shared_magnitudes.iter().filter(|&&edge| {
        edge.endpoints.0 == first_point_guess || edge.endpoints.1 == first_point_guess
    });

    for edge in known_edges_with_first_point {
        // Maybe collect it and sort the array another way
        if !unknown_edges_with_guess
            .clone()
            .any(|&other_edge| edge.magnitude == other_edge.magnitude)
        {
            first_point_guess = t.endpoints.1;
            second_point_guess = t.endpoints.0;
            break;
        }
    }

    let unknown_delta = delta(&first_point_guess, &second_point_guess);
    let (known_x, known_y, known_z) = delta(&first_point, &second_point);

    // This could cause error if two dimensions change at the same rate.
    // Appy transformations to get to 0, 0, 0 from an unknown beacon to the specified coordinate in Scanner 0's perspective
    // This direction/Transformation thing proably also been written more idiomatically

    let mut direction = Transformation {
        x: Direction::X(true),
        y: Direction::Y(true),
        z: Direction::Z(true),
    };

    if unknown_delta.0.abs() == known_x.abs() {
        if unknown_delta.0 != known_x {
            direction.x = Direction::X(false);
        }
    } else if unknown_delta.0.abs() == known_y.abs() {
        if unknown_delta.0 == known_y {
            direction.x = Direction::Y(true);
        } else {
            direction.x = Direction::Y(false);
        }
    } else if unknown_delta.0.abs() == known_z.abs() {
        if unknown_delta.0 == known_z {
            direction.x = Direction::Z(true);
        } else {
            direction.x = Direction::Z(false);
        }
    }

    if unknown_delta.1.abs() == known_x.abs() {
        if unknown_delta.1 == known_x {
            direction.y = Direction::X(true);
        } else {
            direction.y = Direction::X(false);
        }
    } else if unknown_delta.1.abs() == known_y.abs() {
        if unknown_delta.1 != known_y {
            direction.y = Direction::Y(false);
        }
    } else if unknown_delta.1.abs() == known_z.abs() {
        if unknown_delta.1 == known_z {
            direction.y = Direction::Z(true);
        } else {
            direction.y = Direction::Z(false);
        }
    }

    if unknown_delta.2.abs() == known_x.abs() {
        if unknown_delta.2 == known_x {
            direction.z = Direction::X(true);
        } else {
            direction.z = Direction::X(false);
        }
    } else if unknown_delta.2.abs() == known_y.abs() {
        if unknown_delta.2 == known_y {
            direction.z = Direction::Y(true);
        } else {
            direction.z = Direction::Y(false);
        }
    } else if unknown_delta.2.abs() == known_z.abs() && unknown_delta.2 != known_z {
        direction.z = Direction::Z(false);
    }

    let origin = transform(first_point_guess, first_point, &direction);
    // println!("Calculated origin: {:?}", origin);

    unknown.position = Some(origin);
    direction
}

fn calculate_edge_magnitude(a: &Beacon, b: &Beacon) -> f64 {
    (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt()
}

fn delta(a: &Beacon, b: &Beacon) -> (i32, i32, i32) {
    ((a.x - b.x), (a.y - b.y), (a.z - b.z))
}

fn transform(unknown: Beacon, known: Beacon, direction: &Transformation) -> (i32, i32, i32) {
    let (mut x, mut y, mut z) = (known.x, known.y, known.z);

    match direction.x {
        Direction::X(aligned) => {
            if aligned {
                x -= unknown.x
            } else {
                x += unknown.x
            }
        }
        Direction::Y(aligned) => {
            if aligned {
                y -= unknown.x
            } else {
                y += unknown.x
            }
        }
        Direction::Z(aligned) => {
            if aligned {
                z -= unknown.x
            } else {
                z += unknown.x
            }
        }
    }

    match direction.y {
        Direction::X(aligned) => {
            if aligned {
                x -= unknown.y
            } else {
                x += unknown.y
            }
        }
        Direction::Y(aligned) => {
            if aligned {
                y -= unknown.y
            } else {
                y += unknown.y
            }
        }
        Direction::Z(aligned) => {
            if aligned {
                z -= unknown.y
            } else {
                z += unknown.y
            }
        }
    }

    match direction.z {
        Direction::X(aligned) => {
            if aligned {
                x -= unknown.z
            } else {
                x += unknown.z
            }
        }
        Direction::Y(aligned) => {
            if aligned {
                y -= unknown.z
            } else {
                y += unknown.z
            }
        }
        Direction::Z(aligned) => {
            if aligned {
                z -= unknown.z
            } else {
                z += unknown.z
            }
        }
    }

    (x, y, z)
}

fn parse_input(file: &str) -> Vec<Scanner> {
    let mut lines = io::BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty());

    let mut scanners = vec![];

    // Setup the first scanner
    lines.next();
    let mut beacons = vec![];

    for line in lines {
        if line.starts_with("--- scanner ") {
            scanners.push(Scanner {
                beacons,
                position: None,
            });
            beacons = vec![];
        } else {
            let line = line.split(',').map(|s| s.parse::<i32>().unwrap());
            beacons.push(Beacon::from(line));
        }
    }

    scanners.push(Scanner {
        beacons,
        position: None,
    });

    scanners
}

#[test]
fn test() {
    let mut scanners = parse_input("inputs/test");
    assert_eq!(find_beacon_count(&mut scanners), 79);
}

#[test]
fn manhattan_test() {
    let mut scanners = parse_input("inputs/test");

    find_beacon_count(&mut scanners);

    let mut largest_manhattan = get_manhattan_distance(&scanners[0], &scanners[1]);

    for i in 0..scanners.len() - 1 {
        for j in i + 1..scanners.len() {
            let contender = get_manhattan_distance(&scanners[i], &scanners[j]);
            largest_manhattan = cmp::max(contender, largest_manhattan);
        }
    }
    assert_eq!(largest_manhattan, 3621);
}
