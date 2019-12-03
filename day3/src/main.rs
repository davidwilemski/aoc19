use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let mut steps = vec![0, 0];
    let mut coords = vec![(0, 0), (0, 0)];
    let mut wires: Vec<HashSet<(i32, i32)>> = vec![HashSet::new(), HashSet::new()];
    let mut wire_steps: Vec<HashMap<(i32, i32), i32>> = vec![HashMap::new(), HashMap::new()];

    for (wire, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let instrs = line.split(',');
        for instr in instrs {
            let dist = &instr[1..].parse::<i32>().unwrap();
            match &instr[..1] {
                "R" => {
                    for xval in coords[wire].0 + 1..=coords[wire].0 + dist {
                        let pos = (xval, coords[wire].1);
                        if wires[wire].contains(&pos) {
                            println!(
                                "WHOOP, aleady have {:?} with steps {} on wire {}",
                                pos, steps[wire], wire
                            );
                            continue;
                        }
                        steps[wire] += 1;
                        println!("wire {}: pos: {:?}, steps: {}", wire, pos, steps[wire]);
                        wires[wire].insert((xval, coords[wire].1));
                        if let Some(_) = wire_steps[wire].insert(pos, steps[wire]) {
                            panic!("should not happen")
                        }
                    }
                    coords[wire] = (coords[wire].0 + dist, coords[wire].1);
                }
                "L" => {
                    let mut rev_range =
                        (coords[wire].0 - dist..coords[wire].0).collect::<Vec<i32>>();
                    rev_range.reverse();
                    for xval in rev_range.iter() {
                        let pos = (*xval, coords[wire].1);
                        if wires[wire].contains(&pos) {
                            println!(
                                "WHOOP, aleady have {:?} with steps {} on wire {}",
                                pos, steps[wire], wire
                            );
                            continue;
                        }
                        steps[wire] += 1;
                        println!("wire {}: pos: {:?}, steps: {}", wire, pos, steps[wire]);
                        wires[wire].insert((*xval, coords[wire].1));
                        if let Some(_) =
                            wire_steps[wire].insert((*xval, coords[wire].1), steps[wire])
                        {
                            // Value was already there, replace old val
                            panic!("should not happen!");
                            // wire_steps[wire].insert((xval, coords[wire].1), old);
                        }
                    }
                    coords[wire] = (coords[wire].0 - dist, coords[wire].1);
                }
                "U" => {
                    for yval in coords[wire].1 + 1..=coords[wire].1 + dist {
                        let pos = (coords[wire].0, yval);
                        if wires[wire].contains(&pos) {
                            println!(
                                "WHOOP, aleady have {:?} with steps {} on wire {}",
                                pos, steps[wire], wire
                            );
                            continue;
                        }
                        steps[wire] += 1;
                        println!("wire {}: pos: {:?}, steps: {}", wire, pos, steps[wire]);
                        wires[wire].insert((coords[wire].0, yval));
                        if let Some(old) =
                            wire_steps[wire].insert((coords[wire].0, yval), steps[wire])
                        {
                            // Value was already there, replace old val
                            wire_steps[wire].insert((coords[wire].0, yval), old);
                            panic!("should not happen!");
                        }
                    }
                    coords[wire] = (coords[wire].0, coords[wire].1 + dist);
                }
                "D" => {
                    let mut rev_range =
                        (coords[wire].1 - dist..coords[wire].1).collect::<Vec<i32>>();
                    rev_range.reverse();
                    for yval in rev_range.iter() {
                        let pos = (coords[wire].0, *yval);
                        if wires[wire].contains(&pos) {
                            println!(
                                "WHOOP, aleady have {:?} with steps {} on wire {}",
                                pos, steps[wire], wire
                            );
                            continue;
                        }
                        steps[wire] += 1;
                        println!("wire {}: pos: {:?}, steps: {}", wire, pos, steps[wire]);
                        wires[wire].insert((coords[wire].0, *yval));
                        if let Some(old) =
                            wire_steps[wire].insert((coords[wire].0, *yval), steps[wire])
                        {
                            wire_steps[wire].insert((coords[wire].0, *yval), old);
                            panic!("should not happen");
                        }
                    }
                    coords[wire] = (coords[wire].0, coords[wire].1 - dist);
                }
                _ => {
                    panic!("bad input: {}", instr);
                }
            }
        }
    }

    let min_dist = wires[0]
        .intersection(&wires[1])
        .filter(|(x, y)| (*x, *y) != (0, 0))
        .map(|(x, y)| {
            let val = (
                wire_steps[0].get(&(*x, *y)).unwrap(),
                wire_steps[1].get(&(*x, *y)).unwrap(),
            );
            println!("({}, {}): {:?}", x, y, val);
            val
        })
        .min_by_key(|(v1, v2)| **v1 + **v2);

    if let Some(min_dist) = min_dist {
        println!(
            "min dist to intersection: {:?} {}",
            min_dist,
            min_dist.0 + min_dist.1
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_provided_example_data() {}
}
