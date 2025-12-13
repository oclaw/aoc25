use std::vec;

use crate::day;


pub struct Day {
}

#[derive(Clone, Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn distance(a: &Point3D, b: &Point3D) -> f64 {
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn solve_part1(vertexes: &Vec<Point3D>, max_edges: i32) -> Result<String, Box<dyn std::error::Error>> {
    // adjacency matrix
    let mut graph = vec![vec![false; vertexes.len()]; vertexes.len()];

    // put edges between closest points max_edges times
    for _ in 0..max_edges {
        let mut min_distance = std::f64::MAX;
        let mut point_a = 0;
        let mut point_b = 0;
        for i in 0..vertexes.len() {
            for j in (i+1)..vertexes.len() {
                if graph[i][j] {
                    continue;
                }
                let dist = distance(&vertexes[i], &vertexes[j]);
                if dist < min_distance {
                    min_distance = dist;
                    point_a = i;
                    point_b = j;
                }
            }
        }
        graph[point_a][point_b] = true;
        graph[point_b][point_a] = true;
    }

    type HashSet<T> = std::collections::HashSet<T>;

    let mut global_visited = HashSet::new();
    let mut scope_visited = HashSet::new();

    fn dfs(node: usize, graph: &Vec<Vec<bool>>, global_visited: &mut HashSet<usize>, scope_visited: &mut HashSet<usize>) {
        global_visited.insert(node);
        scope_visited.insert(node);
        for neighbor in 0..graph[node].len() {
            if graph[node][neighbor] && !scope_visited.contains(&neighbor) {
                dfs(neighbor, graph, global_visited, scope_visited);
            }
        }
    }
    
    let mut scope_sizes = Vec::new();
    for i in 0..vertexes.len() {
        if !global_visited.contains(&i) {
            dfs(i, &graph, &mut global_visited, &mut scope_visited);
        }
        let scope_size = scope_visited.len();
        scope_sizes.push(scope_size);
        scope_visited.clear();
    }

    scope_sizes.sort();
    scope_sizes.reverse();
    let result = scope_sizes.iter().take(3).product::<usize>();

    Ok(result.to_string())
}
 


impl day::Day for Day {
    fn part1(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        
        // vertex list
        let mut vertex_lines = input.lines().collect::<Vec<&str>>();
        vertex_lines = vertex_lines[..vertex_lines.len()-1].to_vec();
        let vertexes = vertex_lines.iter().map(|line| {
            let coords = line.split(',').collect::<Vec<&str>>();
            Point3D {
                x: coords[0].parse::<i32>().unwrap(),
                y: coords[1].parse::<i32>().unwrap(),
                z: coords[2].parse::<i32>().unwrap(),
            }
        }).collect::<Vec<Point3D>>();


        let max_edges = input.lines().last().unwrap().parse::<i32>().unwrap();
        return solve_part1(&vertexes, max_edges);
    }

    fn part2(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {

        let mut vertex_lines = input.lines().collect::<Vec<&str>>();
        vertex_lines = vertex_lines[..vertex_lines.len()-1].to_vec();
        let vertexes = vertex_lines.iter().map(|line| {
            let coords = line.split(',').collect::<Vec<&str>>();
            Point3D {
                x: coords[0].parse::<i32>().unwrap(),
                y: coords[1].parse::<i32>().unwrap(),
                z: coords[2].parse::<i32>().unwrap(),
            }
        }).collect::<Vec<Point3D>>();

        let mut graph = vec![vec![false; vertexes.len()]; vertexes.len()];


        struct Vector3D {
            a: usize,
            b: usize,
            dist: f64,
        }

        let mut vectors = Vec::new();
        for i in 0..vertexes.len() {
            for j in (i+1)..vertexes.len() {
                let dist = distance(&vertexes[i], &vertexes[j]);
                vectors.push( Vector3D {
                    a: i,
                    b: j,
                    dist,
                });
            }
        }
        vectors.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        // Optimization: we need at least vertexes.len() - 1 edges to connect all points so we can build that many first before checking connectivity
        // this will reduce number of iterations in the main loop
        //
        // This approach might actually be significantly improved with "binary search" of amount of edges since we precisely know order of edges appearance by distance
        // E.g. we can try to build graph with vertexes.len()^2 and keep reducing number of edges until graph becomes disconnected.
        // After we can do some refinements around that point and figure out exact number of edges needed much faster  
        let min_edges_needed = vertexes.len() - 1;
        for vector in vectors.iter().take(min_edges_needed) {
            graph[vector.a][vector.b] = true;
            graph[vector.b][vector.a] = true;
        }
            

        type HashSet<T> = std::collections::HashSet<T>;
        let mut scope_visited = HashSet::new();

        fn dfs(node: usize, graph: &Vec<Vec<bool>>, scope_visited: &mut HashSet<usize>) {
            scope_visited.insert(node);
            for neighbor in 0..graph[node].len() {
                if graph[node][neighbor] && !scope_visited.contains(&neighbor) {
                    dfs(neighbor, graph, scope_visited);
                }
            }
        }

        // put edges between closest points max_edges times
        let mut last_point_a: Option<Point3D>;
        let mut last_point_b: Option<Point3D>;
        let mut edges_put = min_edges_needed; 
        loop {
            let mut min_distance = std::f64::MAX;
            let mut point_a = 0;
            let mut point_b = 0;
            for i in 0..vertexes.len() {
                for j in (i+1)..vertexes.len() {
                    if graph[i][j] {
                        continue;
                    }
                    let dist = distance(&vertexes[i], &vertexes[j]);
                    if dist < min_distance {
                        min_distance = dist;
                        point_a = i;
                        point_b = j;
                    }
                }
            }
            graph[point_a][point_b] = true;
            graph[point_b][point_a] = true;
            edges_put += 1;

            last_point_a = Some(vertexes[point_a].clone());
            last_point_b = Some(vertexes[point_b].clone());

            dfs(0, &graph, &mut scope_visited);
            if scope_visited.len() == vertexes.len() {
                break;
            }

            scope_visited.clear();
        }

    
        println!("All points connected. Two last connected points are: {:?} and {:?}, edges put: {}", last_point_a, last_point_b, edges_put);

        let result = last_point_a.unwrap().x as i64 * last_point_b.unwrap().x as i64;

        Ok(result.to_string())
    }
}

