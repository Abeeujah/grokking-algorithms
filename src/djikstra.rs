use std::collections::{HashMap, HashSet};

fn find_lowest_cost_node(
    costs: &HashMap<String, u32>,
    processed: &HashSet<String>,
) -> Option<String> {
    costs
        .iter()
        .filter_map(|(node, cost)| {
            if cost < &u32::MAX && !processed.contains(node) {
                Some((node, cost))
            } else {
                None
            }
        })
        .min_by_key(|(_, cost)| *cost)
        .map(|(node, _)| node.clone())
}

fn djikstra(graph: &HashMap<String, HashMap<String, u32>>, start: &str) -> HashMap<String, u32> {
    let mut costs: HashMap<String, u32> =
        graph.keys().map(|node| (node.clone(), u32::MAX)).collect();
    costs.insert(start.to_string(), 0);

    let mut processed: HashSet<String> = HashSet::new();

    while let Some(node) = find_lowest_cost_node(&costs, &processed) {
        let cost = costs[&node];
        for (neighbor, weight) in graph[&node].iter() {
            let new_cost = cost + weight;
            if costs[neighbor] > new_cost {
                costs.insert(neighbor.to_owned(), new_cost);
            }
        }
        processed.insert(node);
    }

    costs
}
