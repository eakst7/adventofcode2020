use std::fs;
use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};
use regex::Regex;
use petgraph::algo::dijkstra;

fn find_or_create_node(graph: &mut Graph<String,i32>, s: String) -> petgraph::prelude::NodeIndex {
    let n = graph.node_indices().find(|i| graph[*i] == *s);

    match n {
        None => graph.add_node(s),
        _ => n.unwrap()
    }
}

fn find_node(graph: &Graph<String,i32>, s: String) -> petgraph::prelude::NodeIndex {
    graph.node_indices().find(|i| graph[*i] == *s).unwrap()
}


fn build_graph() -> Graph<String,i32> {
    let contents = fs::read_to_string("input.txt")
        .expect("IO error reading input data");

    let mut graph = Graph::new();

    let re = Regex::new(r"^([\w ]+) bags contain (.*)").unwrap();
    let re2 = Regex::new(r"^(\d+) ([\w ]+) bag").unwrap();
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            let containing_bag = cap[1].to_string();

            let n1 = find_or_create_node(&mut graph, containing_bag);

            for mut d in cap[2].split(",") {
                d = d.trim();
                for cap2 in re2.captures_iter(d) {
                    let count: i32 = cap2[1].to_string().parse().unwrap();
                    let contained_bag = cap2[2].to_string();

                    let n2 = find_or_create_node(&mut graph, contained_bag);

                    graph.add_edge(n2,n1,count); // Part 1
                }
            }
        }
    }

    graph
}

fn part1() {
    let mut graph = build_graph();

    let n = find_node(&mut graph, String::from("shiny gold"));
    let res = dijkstra(&graph, n, None, |_| 1);

    println!("Reachable nodes from shiny gold: {}", res.len()-1);

    //println!("{:?}", Dot::with_config(&graph, &[]));
}

fn count_bags(graph: &Graph<String,i32>, node: petgraph::prelude::NodeIndex) -> i32 {
    if graph.edges(node).count() == 0 {
        println!("{} bag holds no other bags", graph.node_weight(node).unwrap());
        0
    } else {
        let mut count = 0;
        for n in graph.neighbors(node) {

            let edge = graph.find_edge(node, n).unwrap();
            let w = graph.edge_weight(edge).unwrap();
            let c = count_bags(graph, n);

            println!("{} bag holds {} {} bags each of which holds {} other bags", graph.node_weight(node).unwrap(), w, graph.node_weight(n).unwrap(), c);

            count += w * (c + 1);
        }
        count
    }
}

fn part2() {
    let mut graph = build_graph();
    graph.reverse();  // Reverse all the edges

    let n = find_node(&mut graph, String::from("shiny gold"));

    println!("{}", count_bags(&graph, n));

    //println!("{:?}", Dot::with_config(&graph, &[]));
}

fn main() {
    //part1();
    part2();
}