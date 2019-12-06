use std::collections::HashMap;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let orbits = get_orbits(&input);
    println!("{:?}", orbits);
    let result = construct_orbit_tree(&orbits);
    println!("{:?}", result);
    let orbit_count = count_recursive_orbits(result);
    println!("orbit count: {:?}", orbit_count);
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

fn count_recursive_orbits(mut connections: HashMap<&str, Vec<&str>>) -> i32 {
    let com_conns = connections.remove("COM").unwrap();

    // stack of object and level
    let mut stack: Vec<(&str, i32)> = com_conns.iter().map(|v| (*v, 1)).collect();
    let mut count = 0;

    while !stack.is_empty() {
        if let Some((entity, level)) = stack.pop() {
            count += level;
            if let Some(entities) = connections.remove(entity) {
                println!("removed {} and got {:?}", entity, entities);
                for e in entities {
                    stack.push((e, level + 1));
                }
            } // No need for else, just means we can bubble back up
        }
    }
    count
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
        let orbit_count = count_recursive_orbits(result);
        println!("{:?}", orbit_count);

        assert!(orbit_count == 42);
    }
}
