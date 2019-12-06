use std::collections::{HashMap,VecDeque, HashSet};
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let orbits = get_orbits(&input);
    println!("{:?}", orbits);
    let result = construct_orbit_tree(&orbits);
    println!("{:?}", result);
    let data = get_orbital_data(result.clone());
    println!("orbit count: {:?}", data.recursive_orbits);

    // Get the minimum number of orbital transfers required to go from YOU to SAN
    // First, we need the first common node...
    let mut last_common_ent : usize = usize::max_value();
    for (idx, (you_ent, san_ent)) in data.path_to_you.iter().zip(data.path_to_san.iter()).enumerate() {
       if you_ent == san_ent {
           last_common_ent = idx;
           continue
       }
       break
    }

    println!("last common ent: {}, idx: {}", data.path_to_you[last_common_ent], last_common_ent);
    println!("last common ent: {}, idx: {}", data.path_to_san[last_common_ent], last_common_ent);

    println!("distance from last common ent to YOU: {}", data.path_to_you.len() - last_common_ent - 1);
    println!("distance from last common ent to SAN: {}", data.path_to_san.len() - last_common_ent - 1);

    // The above method failed to construct accurate pathes for some reason, so reversing the
    // approach to traverse from the end of the path back to COM...  It isn't really clear to me
    // why the other approach to calculating the paths didn't work and I'd like to find time to
    // investigate more but for now let's solve...


    let rev_orbits = construct_reverse_orbit_tree(&orbits);
    let mut path_from_you : Vec<&str> = Vec::new();
    let mut current = "YOU";
    while current != "COM" {
        path_from_you.push(current);
        current = rev_orbits.get(current).unwrap();
    }
    let mut path_from_san : Vec<&str> = Vec::new();
    current = "SAN";
    while current != "COM" {
        path_from_san.push(current);
        current = rev_orbits.get(current).unwrap();
    }

    let mut san_set : HashSet<&str> = HashSet::new();
    let mut you_set = HashSet::new();
    san_set.extend(path_from_san.iter());
    you_set.extend(path_from_you.iter());
    println!("len of path: {}, path from you: {:?}", path_from_you.len(), path_from_you);
    println!("len of path: {}, path from san: {:?}", path_from_san.len(), path_from_san);

    // multiply by two to remove the intersecting bits.
    // subtract by two to remove SAN and YOU.
    println!("difference between sets: {}", san_set.len() + you_set.len() - (san_set.intersection(&you_set).count() * 2) - 2);
    // subtract by two to remove SAN and YOU.
    println!("difference between sets: {}", san_set.symmetric_difference(&you_set).count() - 2);

    Ok(())
}

fn get_orbits(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|l| l.split(')').collect::<Vec<&str>>())
        .map(|v| (v[0], v[1]))
        .collect::<Vec<(&str, &str)>>()
}

fn construct_orbit_tree<'a>(orbits: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    for (entity, orbiter) in orbits.iter() {
        if let Some(orbiters) = connections.get_mut(entity) {
            orbiters.push(orbiter);
        } else {
            connections.insert(entity, vec![orbiter]);
        }
    }
    connections
}

fn construct_reverse_orbit_tree<'a>(orbits: &Vec<(&'a str, &'a str)>) -> HashMap<&'a str, &'a str> {
    let mut connections: HashMap<&str, &str> = HashMap::new();

    for (entity, orbiter) in orbits.iter() {
        connections.insert(orbiter, entity);
    }
    connections
}

#[derive(Debug)]
struct OrbitalData<'a> {
    recursive_orbits: i32,
    path_to_you: Vec<&'a str>,
    path_to_san: Vec<&'a str>,
}

fn get_orbital_data<'a>(mut connections: HashMap<&'a str, Vec<&'a str>>) -> OrbitalData<'a> {
    let com_conns = connections.remove("COM").unwrap();

    // stack of object and level
    let mut stack: Vec<(&str, i32)> = com_conns.iter().map(|v| (*v, 1)).collect();
    let mut count = 0;
    let mut current_path = vec!["COM"];
    let mut path_to_you = vec![];
    let mut path_to_san = vec![];

    while !stack.is_empty() {
        if let Some((entity, level)) = stack.pop() {
            count += level;
            current_path.push(entity);
            // if entity == "SAN" || entity == "YOU" {
            if entity == "YOU" {
                println!("path found! {:?}", current_path);
                path_to_you = current_path.clone();
            }
            if entity == "SAN" {
                println!("path found! {:?}", current_path);
                path_to_san = current_path.clone();
            }
            if let Some(entities) = connections.remove(entity) {
                println!("removed {} and got {:?}", entity, entities);
                for e in entities {
                    stack.push((e, level + 1));
                }
            } else {
                current_path.pop();
            }
        }
    }
    OrbitalData { recursive_orbits: count, path_to_you, path_to_san }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_provided_example_data() {
        let example = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";
        let orbits = get_orbits(example);
        println!("{:?}", orbits);
        let result = construct_orbit_tree(&orbits);
        println!("{:?}", result);
        let data = get_orbital_data(result);
        println!("{:?}", data);

        assert!(data.recursive_orbits == 42);
    }
}
