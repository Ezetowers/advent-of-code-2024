use log::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{BufRead, BufReader};

use advent_of_code_2024::common;

/*---------------------------------------------------------------------------*/

const TURN_WEIGHT: u32 = 1000;
const ADVANCE_WEIGHT: u32 = 1;
const INFINITY: u32 = 99999999;
const UNDEFINED: u32 = 99999;

/*---------------------------------------------------------------------------*/

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct State {
    pos: (usize, usize),
    direction: (i32, i32),
    index: usize,
    cost: u32,
}

/*---------------------------------------------------------------------------*/

fn main() -> Result<(), Box<dyn Error>> {
    let _log2 = common::setup_logger();
    let reader = BufReader::new(common::setup_input()?);
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut start_index: usize = 0;
    let mut end_index: usize = 0;

    let mut x = 0;
    let mut node_counter: usize = 0;
    let mut coord_to_index: HashMap<(usize, usize), usize> = HashMap::new();
    let mut index_to_coord: HashMap<usize, (usize, usize)> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        maze.push(line.chars().collect());

        let mut y = 0;
        for character in line.chars() {
            if character == '.' {
                coord_to_index.insert((x, y), node_counter);
                index_to_coord.insert(node_counter, (x, y));
                node_counter += 1;
            }
            if character == 'S' {
                coord_to_index.insert((x, y), node_counter);
                index_to_coord.insert(node_counter, (x, y));
                start_index = node_counter;
                debug!("Start index: {}", start_index);
                node_counter += 1;
                maze[x][y] = '.';
            }
            if character == 'E' {
                coord_to_index.insert((x, y), node_counter);
                index_to_coord.insert(node_counter, (x, y));
                end_index = node_counter;
                debug!("End index: {}", end_index);
                node_counter += 1;
                maze[x][y] = '.';
            }
            y = y + 1;
        }
        x = x + 1;
    }

    let mut adj_matrix = vec![vec![INFINITY; node_counter]; node_counter];
    info!("Node counter: {}", node_counter);

    // The idea is to use dijkstra shortest path algorithm. In order to do it, we need to
    // get the adjacent matrix associated with the maze. We need to identify turns, and
    // set them with the proper weight
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
                    adj_matrix[*node_index][up_adj_index] = ADVANCE_WEIGHT;
                }

                if down {
                    adj_matrix[*node_index][down_adj_index] = ADVANCE_WEIGHT;
                }

                if right {
                    adj_matrix[*node_index][right_adj_index] = ADVANCE_WEIGHT;
                }

                if left {
                    adj_matrix[*node_index][left_adj_index] = ADVANCE_WEIGHT;
                }
            }
        }
    }

    for x in 0..adj_matrix.len() {
        trace!("[Node adjacents {}] {:?}", x, adj_matrix[x]);
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
    //

    // Use Dijkstra to search minimum path (for now)
    let node_counter = adj_matrix.len();
    let mut dist: Vec<Option<State>> = vec![None; node_counter];
    let mut prev = vec![UNDEFINED as usize; node_counter];
    let mut q: HashSet<usize> = HashSet::new();
    for i in 0..node_counter {
        q.insert(i);
    }
    dist[start_index as usize] = Some(State {
        pos: index_to_coord[&start_index],
        index: start_index,
        direction: (0, 1),
        cost: 0,
    });

    while q.len() != 0 {
        // Get vertex in Q with min distance to source (there can be more than one)
        let mut min_vertexes: Vec<State> = Vec::new();
        let mut min_distance = INFINITY;
        for element in q.iter() {
            if let Some(state) = &dist[*element] {
                if state.cost < min_distance {
                    min_distance = state.cost;
                }
            }
        }

        for element in q.iter() {
            if let Some(state) = &dist[*element] {
                if state.cost == min_distance {
                    min_vertexes.push(state.clone());
                }
            }
        }

        // Remove u from Q
        for vertex in min_vertexes.iter() {
            trace!("Removing {:?}", (vertex.pos.0 + 1, vertex.pos.1 + 1));
            q.remove(&vertex.index);
        }

        // for each neighbor v of u still in Q
        for vertex in min_vertexes.iter() {
            for i in 0..node_counter {
                // Get neighbors
                if adj_matrix[vertex.index][i] != INFINITY {
                    // Do not compute the vertex if not in Q
                    if !q.contains(&i) {
                        continue;
                    }
                    // alt ← dist[u] + length(u, v)
                    // let alt = dist[*vertex] + adj_matrix[*vertex][i];
                    //
                    // NOTE: This is where regular dijkstra does not work. We need to know from
                    // from which is the direction that we had and which is direction where we are
                    // heading
                    let mut alt = vertex.cost + ADVANCE_WEIGHT;
                    let new_pos = (
                        vertex.pos.0 as i32 + vertex.direction.0,
                        vertex.pos.1 as i32 + vertex.direction.1,
                    );
                    let adj_pos = index_to_coord[&i];

                    let mut new_direction = vertex.direction;
                    if new_pos.0 as usize != adj_pos.0 || new_pos.1 as usize != adj_pos.1 {
                        // The adjacent node does not match our direction
                        alt = vertex.cost + TURN_WEIGHT + ADVANCE_WEIGHT;
                        new_direction = (
                            adj_pos.0 as i32 - vertex.pos.0 as i32,
                            adj_pos.1 as i32 - vertex.pos.1 as i32,
                        );
                    }

                    trace!(
                        "Alt from {:?} to  {:?}: {}. Vertex dir: {:?}",
                        (vertex.pos.0 + 1, vertex.pos.1 + 1),
                        (adj_pos.0 + 1, adj_pos.1 + 1),
                        alt,
                        vertex.direction,
                    );
                    match dist[i] {
                        Some(mut state) => {
                            if alt < state.cost {
                                state.cost = alt;
                                state.direction = new_direction;
                                dist[i] = Some(state.clone());
                                prev[i] = vertex.index;
                            }
                        }
                        None => {
                            let state = State {
                                cost: alt,
                                direction: new_direction,
                                index: i,
                                pos: adj_pos,
                            };
                            dist[i] = Some(state);
                            prev[i] = vertex.index;
                        }
                    };
                }
            }
        }
    }

    // Draw shortest path
    maze[index_to_coord[&start_index].0][index_to_coord[&start_index].1] = 'S';
    maze[index_to_coord[&end_index].0][index_to_coord[&end_index].1] = 'E';

    let mut current_index = end_index;
    while prev[current_index] != UNDEFINED as usize {
        let cell = index_to_coord[&current_index];
        debug!(
            "Cell: {:?} - Distance: {}",
            (cell.0 + 1, cell.1 + 1),
            dist[current_index].unwrap().cost,
        );
        maze[cell.0][cell.1] = '*';
        current_index = prev[current_index];
    }

    debug!("Maze!");
    for i in 0..maze.len() {
        let row: String = maze[i].clone().into_iter().collect();
        debug!("{}", row);
    }

    info!("Day X - Exercise Y. Result: {:?}", dist[end_index as usize]);
    Ok(())
}
