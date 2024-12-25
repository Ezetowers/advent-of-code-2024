use log::*;
use std::error::Error;
use std::io::{BufRead, BufReader};

use petgraph::graph::Graph;
use std::collections::{HashMap, HashSet};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const TURN_WEIGHT: u32 = 1000;
const ADVANCE_WEIGHT: u32 = 1;
const INFINITY: u32 = 99999999;

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start_index: u32 = 0;
    let mut end_index: u32 = 0;

    let mut x = 0;
    let mut node_counter: usize = 0;
    let mut coord_to_index: HashMap<(usize, usize), u32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        maze.push(line.chars().collect());

        let mut y = 0;
        for character in line.chars() {
            if character == '.' {
                coord_to_index.insert((x, y), node_counter as u32);
                node_counter += 1;
            }
            if character == 'S' {
                coord_to_index.insert((x, y), node_counter as u32);
                start_index = node_counter as u32;
                debug!("Start index: {}", start_index);
                node_counter += 1;
            }
            if character == 'E' {
                coord_to_index.insert((x, y), node_counter as u32);
                end_index = node_counter as u32;
                debug!("End index: {}", end_index);
                node_counter += 1;
            }
            y = y + 1;
        }
        x = x + 1;
    }

    let mut adj_matrix = vec![vec![INFINITY; node_counter]; node_counter];
    info!("Node counter: {}", node_counter);
    trace!("Maze");
    for x in 0..maze.len() {
        for y in 0..maze[x].len() {
            if maze[x][y] == '.' {
                // Add adjacent nodes and edges
                let up = maze[x - 1][y] == '.';
                let up_adj_index = coord_to_index.entry((x - 1, y)).or_default().clone();
                let down = maze[x + 1][y] == '.';
                let down_adj_index = coord_to_index.entry((x + 1, y)).or_default().clone();
                let right = maze[x][y + 1] == '.';
                let right_adj_index = coord_to_index.entry((x, y + 1)).or_default().clone();
                let left = maze[x][y - 1] == '.';
                let left_adj_index = coord_to_index.entry((x, y - 1)).or_default().clone();
                let node_index = coord_to_index.entry((x, y)).or_default();

                if up {
                    adj_matrix[*node_index as usize][up_adj_index as usize] = ADVANCE_WEIGHT;
                    if !down {
                        adj_matrix[*node_index as usize][up_adj_index as usize] += TURN_WEIGHT;
                    }
                }

                if down {
                    adj_matrix[*node_index as usize][down_adj_index as usize] = ADVANCE_WEIGHT;
                    if !up {
                        adj_matrix[*node_index as usize][down_adj_index as usize] += TURN_WEIGHT;
                    }
                }

                if right {
                    adj_matrix[*node_index as usize][right_adj_index as usize] = ADVANCE_WEIGHT;
                    if !left {
                        adj_matrix[*node_index as usize][right_adj_index as usize] += TURN_WEIGHT;
                    }
                }

                if left {
                    adj_matrix[*node_index as usize][left_adj_index as usize] = ADVANCE_WEIGHT;
                    if !right {
                        adj_matrix[*node_index as usize][left_adj_index as usize] += TURN_WEIGHT;
                    }
                }
            }
        }
    }

    // * dist is an array that contains the current distances from the source to other vertices, i.e. dist[u] is the current distance from the source to the vertex u.
    // * The prev array contains pointers to previous-hop nodes on the shortest path from source to the given vertex (equivalently, it is the next-hop on the path from the given vertex to the source)
    // * The code u ← vertex in Q with min dist[u], searches for the vertex u in the vertex set Q that has the least dist[u] value
    // * Graph.Edges(u, v) returns the length of the edge joining (i.e. the distance between) the two neighbor-nodes u and v
    // * The variable alt on line 14 is the length of the path from the source node to the neighbor node v if it were to go through u
    //   * If this path is shorter than the current shortest path recorded for v, then the distance of v is updated to alt

    // 1  function Dijkstra(Graph, source):
    //  2
    //  3      for each vertex v in Graph.Vertices:
    //  4          dist[v] ← INFINITY
    //  5          prev[v] ← UNDEFINED
    //  6          add v to Q
    //  7      dist[source] ← 0
    //  8
    //  9      while Q is not empty:
    // 10          u ← vertex in Q with minimum dist[u]
    // 11          remove u from Q
    // 12
    // 13          for each neighbor v of u still in Q:
    // 14              alt ← dist[u] + Graph.Edges(u, v)
    // 15              if alt < dist[v]:
    // 16                  dist[v] ← alt
    // 17                  prev[v] ← u
    // 18
    // 19      return dist[], prev[]

    // Use Dijkstra to search minimum path (for now)
    let mut dist: Vec<u32> = vec![INFINITY; node_counter];
    let mut prev: Vec<usize> = Vec::new();
    let mut q: HashSet<usize> = HashSet::new();
    for i in 0..node_counter {
        q.insert(i);
    }
    dist[start_index as usize] = 0;
    while q.len() != 0 {
        // Get vertex in Q with min distance to source (there can be more than one)
        let mut min_vertexes: Vec<usize> = Vec::new();
        let mut min_distance = INFINITY;
        for element in q.iter() {
            if dist[*element] < min_distance {
                min_distance = dist[*element];
            }
        }

        for element in q.iter() {
            if dist[*element] == min_distance {
                min_vertexes.push(*element);
            }
        }

        // Remove u from Q
        for vertex in min_vertexes.iter() {
            trace!("Removing {}", *vertex);
            q.remove(vertex);
        }

        // for each neighbor v of u still in Q
        for vertex in min_vertexes.iter() {
            for i in 0..node_counter {
                // Get neighbors
                if adj_matrix[start_index as usize][i] != INFINITY {
                    // alt ← dist[u] + length(u, v)
                    let alt = dist[*vertex] + adj_matrix[start_index as usize][i];
                    trace!("Alt: {}", alt);
                    if alt < dist[i] {
                        dist[i] = alt;
                        prev.push(i);
                    }
                }
            }
        }
    }

    info!("Day X - Exercise Y. Result: {}", 0);
    info!("{:?}", dist);
    Ok(())
}
