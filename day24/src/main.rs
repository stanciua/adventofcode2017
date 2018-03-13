extern crate petgraph;

use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};
use petgraph::graph::Graph;
use petgraph::visit::Dfs;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut ports = HashSet::new();
    for line in input_txt.lines() {
        let p = line.split('/').collect::<Vec<_>>();
        ports.insert((p[0].parse::<i32>().unwrap(), p[1].parse::<i32>().unwrap()));
    }

    let starting_ports_set = ports
        .iter()
        .cloned()
        .filter(|v| v.0 == 0 || v.1 == 0)
        .collect::<HashSet<_>>();
    let mut remaining_ports = ports
        .difference(&starting_ports_set)
        .cloned()
        .collect::<HashSet<_>>();

    // for each starting port try to calculate the strongest bridge
    let mut strengths = Vec::new();
    for port in starting_ports_set {
        remaining_ports.insert(port);
        let graph = build_port_graph(&remaining_ports);
        strengths.push(calculate_strength(port, &graph));
        remaining_ports.remove(&port);
    }

    println!(
        "The strongest bridge is: {:?}",
        strengths.iter().max().unwrap()
    );
}

fn get_adjacent_list(port: (i32, i32), ports: &HashSet<(i32, i32)>) -> Vec<(i32, i32)> {
    ports.iter().fold(Vec::new(), |mut acc, v| {
        if port != *v && (port.0 == v.0 || port.1 == v.1 || port.0 == v.1 || port.1 == v.0) {
            acc.push(*v);
        }
        acc
    })
}

fn build_port_graph(ports: &HashSet<(i32, i32)>) -> Graph<(i32, i32), (i32, i32)> {
    let (node_idxs, mut graph) = ports.iter().fold(
        (HashMap::new(), Graph::<(i32, i32), (i32, i32)>::new()),
        |mut acc, v| {
            let idx = acc.1.add_node(*v);
            acc.0.entry(v).or_insert(idx);
            acc
        },
    );

    let edges = ports
        .iter()
        .map(|p| (*p, get_adjacent_list(*p, ports)))
        .map(|(p, adjacent_list)| adjacent_list.iter().map(|v| (p, *v)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|i| i.iter())
        .map(|&(n1, n2)| (node_idxs[&n1], node_idxs[&n2]))
        .collect::<Vec<_>>();

    graph.extend_with_edges(edges.as_slice());

    graph
}

fn calculate_strength(port: (i32, i32), graph: &Graph<(i32, i32), (i32, i32)>) -> i32 {
    let mut length = 0;
    let starting_node = graph
        .node_indices()
        .find(|&i| *graph.node_weight(i).unwrap() == port)
        .unwrap();

    let mut dfs = Dfs::new(&graph, starting_node);

    while let Some(node) = dfs.next(&graph) {
        if let Some(weight) = graph.node_weight(node) {
            length += weight.0 + weight.1;
        }
    }

    length
}
