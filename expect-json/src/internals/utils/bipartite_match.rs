// Find the best matching in a bipartite graph using the Hopcroft-Karp algorithm.
pub fn bipartite_match(size: usize, edges: &[(usize, usize)]) -> Vec<Option<usize>> {
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); size];
    for &(u, v) in edges {
        adj_list[u].push(v);
    }

    let mut match_to: Vec<Option<usize>> = vec![None; size];

    for u in 0..size {
        let mut visited = vec![false; size];
        bpm_dfs(u, &adj_list, &mut visited, &mut match_to);
    }

    match_to
}

fn bpm_dfs(
    u: usize,
    adj_list: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    match_to: &mut Vec<Option<usize>>,
) -> bool {
    for &v in &adj_list[u] {
        if !visited[v] {
            visited[v] = true;

            if match_to[v].is_none() || bpm_dfs(match_to[v].unwrap(), adj_list, visited, match_to) {
                match_to[v] = Some(u);
                return true;
            }
        }
    }
    false
}
