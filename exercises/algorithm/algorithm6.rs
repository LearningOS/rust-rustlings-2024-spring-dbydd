/*
    dfs
    This problem requires you to implement a basic DFS traversal
*/

use std::collections::{HashSet, VecDeque};

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        visited.insert(v);
        visit_order.push(v);

        for &neighbor in &self.adj[v] {
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
        // let mut visit_stack: VecDeque<usize> = VecDeque::new();
        // self.adj[v]
        //     .iter()
        //     .rev()
        //     .for_each(|item| visit_stack.push_back(*item));
        // visited.insert(v);
        // visit_order.push(v);
        //
        // loop {
        //     let item_o = &visit_stack.pop_back();
        //     if let Some(item) = item_o
        //         && !visited.contains(&item)
        //     {
        //         visit_order.push(*item);
        //         visited.insert(*item);
        //
        //         self.adj[*item]
        //             .clone()
        //             .iter()
        //             .filter(|item| !(visited.contains(item)))
        //             .map(|i| (*i).clone())
        //             .rev()
        //             .for_each(|ele| {
        //                 visit_stack.push_back(ele);
        //             });
        //     } else {
        //         break;
        //     }
        // }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }
}
