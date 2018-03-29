extern crate petgraph;

use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};
use petgraph::graph::Graph;
use petgraph::visit::Dfs;
use std::cmp::Ordering;
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::algo::*;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut ports = Vec::new();
    for line in input_txt.lines() {
        let p = line.split('/').collect::<Vec<_>>();
        ports.push((p[0].parse::<i32>().unwrap(), p[1].parse::<i32>().unwrap()));
    }

    let starting_ports_set = ports
        .iter()
        .cloned()
        .filter(|v| v.0 == 0 || v.1 == 0)
        .map(|v| if v.0 != 0 { (v.1, v.0) } else { v })
        .collect::<Vec<_>>();
    let mut remaining_ports = ports
        .iter()
        .cloned()
        .filter(|v| v.0 != 0 && v.1 != 0)
        .collect::<Vec<_>>();

    // for each starting port try to calculate the strongest bridge
    let mut strengths = Vec::new();
    for port in starting_ports_set {
        remaining_ports.insert(0, port);
        // println!("{:?}", remaining_ports);
        let graph = build_port_graph(&remaining_ports);
        println!("--------------------------------------------------");
        strengths.push(calculate_strength(port, &graph));
        remaining_ports.remove(0);
    }

    println!("{:?}", strengths);
    println!(
        "The strongest bridge is: {:?}",
        strengths.iter().max().unwrap()
    );
}

fn get_adjacent_list(port: (i32, i32), ports: &[(i32, i32)]) -> Vec<(i32, i32)> {
    ports.iter().fold(Vec::new(), |mut acc, v| {
        if port != *v && (port.0 == v.0 || port.1 == v.1 || port.0 == v.1 || port.1 == v.0) {
            acc.push(*v);
        }
        acc
    })
}

fn build_port_graph(ports: &[(i32, i32)]) -> Graph<(i32, i32), (i32, i32)> {
    let mut union = ports
        .iter()
        .cloned()
        .map(|p| (p, get_adjacent_list(p, ports)))
        .map(|(p, v)| v.into_iter().map(move |n| (p, n)).collect::<Vec<_>>())
        .flat_map(|i| i.into_iter())
        .map(|p| if p.0 > p.1 { (p.1, p.0) } else { p })
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    union.sort_by(|a, b| {
        let order = a.0.cmp(&b.0);
        match order {
            Ordering::Equal => a.1.cmp(&b.1),
            _ => order,
        }
    });

    let (node_idxs, mut graph) = ports.iter().fold(
        (HashMap::new(), Graph::<(i32, i32), (i32, i32)>::new()),
        |mut acc, v| {
            let idx = acc.1.add_node(*v);
            acc.0.entry(v).or_insert(idx);
            acc
        },
    );

    let edges = union
        .iter()
        .map(|&(n1, n2)| (node_idxs[&n1], node_idxs[&n2]))
        .collect::<Vec<_>>();

    graph.extend_with_edges(edges.as_slice());

    graph
}

fn calculate_strength(port: (i32, i32), graph: &Graph<(i32, i32), (i32, i32)>) -> i32 {
    //     println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // println!("{:?}", graph);
    // println!("{:?}", is_cyclic_directed(&graph));
    println!(
        "neighbors: {:?}",
        graph
            .node_indices()
            .inspect(|n| println!("{:?} -> ", graph.node_weight(*n).unwrap()))
            .map(|n| graph.neighbors(n).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    );
    if let Ok(v) = toposort(&graph, None) {
        // get the position of node 0 in topological order
        if let Some(pos) = v.iter()
            .position(|n| if n.index() == 0 { true } else { false })
        {
            return v.into_iter()
                .skip(pos)
                // .inspect(|&n| println!("{:?} -> {:?}", n, graph.node_weight(n)))
                .fold(0, |mut acc, n| {
                    let weight = graph.node_weight(n).unwrap();
                    acc += weight.0 + weight.1;
                    acc
                });
        }
    }

    0
}
