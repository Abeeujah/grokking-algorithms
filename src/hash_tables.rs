use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub fn has_voted<T, V>(key: T, map: &HashMap<T, V>) -> Result<(), String>
where
    T: Hash + Eq,
{
    match map.contains_key(&key) {
        true => Err("Already Voted".to_string()),
        false => Ok(()),
    }
}

fn cached_data<T>(key: T) -> T
where
    T: Eq + Hash + Copy,
{
    let mut map: HashMap<T, T> = HashMap::new();
    match map.get(&key) {
        Some(val) => *val,
        None => match map.insert(key, key) {
            None => *map.get(&key).expect("Value cannot be None"),
            Some(val) => val,
        },
    }
}

fn ends_with_m<'a>(name: &'a [&str]) -> (bool, &'a str) {
    match name.into_iter().find(|x| x.ends_with('m')) {
        None => (false, ""),
        Some(name) => (true, name),
    }
}

fn search_neighbors() {
    let mut neighbors = HashMap::new();
    neighbors.insert("you", vec!["alice", "bob", "peggy"]);
    neighbors.insert("bob", vec!["anuj", "peggy"]);
    neighbors.insert("alice", vec!["peggy"]);
    neighbors.insert("claire", vec!["thom", "jonny"]);
    neighbors.insert("anuj", vec![]);
    neighbors.insert("peggy", vec![]);
    neighbors.insert("thom", vec![]);
    neighbors.insert("jonny", vec![]);
    let mut search_queue: VecDeque<Vec<&str>> = VecDeque::new();
    search_queue.push_back(neighbors["you"].clone());

    while !search_queue.is_empty() {
        let people = search_queue
            .pop_back()
            .expect("Search Queue cannot be empty");
        let (found, name) = ends_with_m(&people[..]);
        if found {
            println!("{name} is a mango seller.");
        } else {
            for person in people {
                if search_queue.contains(&neighbors[person]) {
                    continue;
                }
                search_queue.push_back(neighbors[person].clone())
            }
        }
    }
}

fn breadth_first_search<T>(graph: HashMap<T, Vec<T>>, start: &T) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.clone());
    queue.push_back(start.clone());

    let mut result = Vec::new();
    while let Some(current) = queue.pop_front() {
        result.push(current.clone());

        for neighbor in graph.get(&current).unwrap_or(&Vec::new()) {
            if visited.insert(neighbor.clone()) {
                queue.push_back(neighbor.clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = HashMap::new();
        graph.insert('A', vec!['B', 'C']);
        graph.insert('B', vec!['A', 'D', 'E']);
        graph.insert('C', vec!['A', 'F']);
        graph.insert('D', vec!['B']);
        graph.insert('E', vec!['B', 'F']);
        graph.insert('F', vec!['C', 'E']);
        assert_eq!(
            breadth_first_search(graph, &'A'),
            vec!['A', 'B', 'C', 'D', 'E', 'F']
        );
    }

    #[test]
    fn test_has_voted() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(Ok(()), has_voted("value", &map));
        assert_eq!(Err("Already Voted".to_string()), has_voted("key", &map));
    }

    #[test]
    fn test_cache_data() {
        assert_eq!("Cow", cached_data("Cow"));
    }
}
