pub fn dfs_recursive<F>(start: usize, graph: &[Vec<usize>], visited: &mut [bool], visit: &mut F)
where
    F: FnMut(usize),
{
    fn traverse<F>(node: usize, graph: &[Vec<usize>], visited: &mut [bool], visit: &mut F)
    where
        F: FnMut(usize),
    {
        if visited[node] {
            return;
        }

        visited[node] = true;
        visit(node);

        for &next in &graph[node] {
            if !visited[next] {
                traverse(next, graph, visited, visit);
            }
        }
    }

    traverse(start, graph, visited, visit);
}
