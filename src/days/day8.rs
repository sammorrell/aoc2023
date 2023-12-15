use num::integer::lcm;
use rayon::prelude::*;
use regex::Regex;
use std::path::Path;

const INPUT: &str = include_str!("../../data/day8/data.txt");

pub fn parse_input() -> (String, Vec<String>, Vec<(String, String)>) {
    let input_str = INPUT;
    let instructions = input_str
        .lines()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .to_string();
    let nodes = input_str
        .lines()
        .skip(2)
        .map(|line| line.split(" = ").next().unwrap().to_string())
        .collect();
    let match_patter = Regex::new(r"[ ]*([0-9A-Z]+), ([0-9A-Z]+)\)[ ]*").unwrap();
    let edges = input_str
        .lines()
        .skip(2)
        .map(|line| {
            let caps = match_patter.captures(line).unwrap();
            (caps[1].to_string(), caps[2].to_string())
        })
        .collect();
    (instructions, nodes, edges)
}

pub fn match_edges_to_nodes(
    nodes: &Vec<String>,
    edges: &Vec<(String, String)>,
) -> Vec<(usize, usize)> {
    edges
        .iter()
        .map(|(left, right)| {
            let left_index = nodes.iter().position(|node| node == left).unwrap();
            let right_index = nodes.iter().position(|node| node == right).unwrap();
            (left_index, right_index)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day8_part1() {
        let (instructions, nodes, edges) = parse_input();
        let matched_edges = match_edges_to_nodes(&nodes, &edges);

        let mut i = 0;
        let mut pos = nodes.iter().position(|node| node == "AAA").unwrap();
        let dest = nodes.iter().position(|node| node == "ZZZ").unwrap();
        while pos != dest {
            let instruction = instructions.chars().nth(i % instructions.len()).unwrap();
            pos = match instruction {
                'L' => matched_edges[pos].0,
                'R' => matched_edges[pos].1,
                _ => panic!("Unknown direction"),
            };
            i += 1;
        }

        assert_eq!(i, 11911);
    }

    #[test]
    fn day8_part2() {
        let (instructions, nodes, edges) = parse_input();
        let matched_edges = match_edges_to_nodes(&nodes, &edges);
        let starts: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|node| node.1.chars().last().unwrap() == 'A')
            .map(|node| node.0)
            .collect();
        let ends: Vec<usize> = nodes
            .iter()
            .enumerate()
            .filter(|node| node.1.chars().last().unwrap() == 'Z')
            .map(|node| node.0)
            .collect();

        let distances: Vec<usize> = starts
            .par_iter()
            .map(|start| {
                let mut pos = *start;
                let mut i = 0;
                while !ends.contains(&pos) {
                    let instruction = instructions.chars().nth(i % instructions.len()).unwrap();
                    pos = match instruction {
                        'L' => matched_edges[pos].0,
                        'R' => matched_edges[pos].1,
                        _ => panic!("Unknown direction"),
                    };
                    i += 1;
                }
                i
            })
            .collect();

        let min_dist = distances.iter().fold(1, |accum, dist| lcm(accum, *dist));

        assert_eq!(min_dist, 10151663816849);
    }
}
