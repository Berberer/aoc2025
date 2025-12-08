use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type NodeId = (usize, usize);
type AdjacencyList = HashMap<NodeId, (Option<NodeId>, Option<NodeId>)>;

struct Tree {
    root_node: NodeId,
    adjacency_list: AdjacencyList,
}

fn parse_input(input: &str) -> (usize, Vec<Vec<char>>) {
    (
        input.trim().find('S').unwrap(),
        input
            .trim()
            .lines()
            .map(|line| line.trim().chars().collect())
            .collect(),
    )
}

fn find_next_split(
    (mut layer, index): (usize, usize),
    map: &[Vec<char>],
) -> Option<(usize, usize)> {
    while layer < map.len() {
        if map[layer][index] == '^' {
            return Some((layer, index));
        } else {
            layer += 1;
        }
    }

    None
}

fn construct_beam_split_tree(start_index: usize, map: &[Vec<char>]) -> Tree {
    let root_split = find_next_split((0, start_index), map).unwrap();

    let mut splits_tree: HashMap<NodeId, (Option<NodeId>, Option<NodeId>)> = HashMap::new();
    let mut open_list = vec![root_split];

    while let Some(next_node) = open_list.pop() {
        let next_left = find_next_split((next_node.0 + 2, next_node.1 - 1), map);
        let next_right = find_next_split((next_node.0 + 2, next_node.1 + 1), map);

        splits_tree.insert(next_node, (next_left, next_right));

        if let Some(left_child_node) = next_left
            && !open_list.contains(&left_child_node)
        {
            open_list.insert(0, left_child_node);
        }
        if let Some(right_child_node) = next_right
            && !open_list.contains(&right_child_node)
        {
            open_list.insert(0, right_child_node);
        }
    }

    Tree {
        root_node: root_split,
        adjacency_list: splits_tree,
    }
}

fn find_beam_ends(adjacency_list: &AdjacencyList) -> HashSet<(NodeId, u64)> {
    adjacency_list
        .iter()
        .filter(|(_, (left, right))| left.is_none() || right.is_none())
        .map(|(node, (left, right))| {
            (
                *node,
                match (left, right) {
                    (None, Some(_)) => 1,
                    (Some(_), None) => 1,
                    _ => 2,
                },
            )
        })
        .collect()
}

fn invert_adjacency_list(adjacency_list: &AdjacencyList) -> HashMap<NodeId, HashSet<NodeId>> {
    let mut inverted_adjacency_list = HashMap::new();

    for (node, (left, right)) in adjacency_list {
        if let Some(left_child) = left {
            inverted_adjacency_list
                .entry(*left_child)
                .and_modify(|parents: &mut HashSet<NodeId>| {
                    parents.insert(*node);
                })
                .or_insert(HashSet::from([*node]));
        }

        if let Some(right_child) = right {
            inverted_adjacency_list
                .entry(*right_child)
                .and_modify(|parents: &mut HashSet<NodeId>| {
                    parents.insert(*node);
                })
                .or_insert(HashSet::from([*node]));
        }
    }

    inverted_adjacency_list
}

fn count_paths(split_tree: &Tree) -> HashMap<NodeId, u64> {
    let inverted_adjacency_list = invert_adjacency_list(&split_tree.adjacency_list);

    let mut path_counts = HashMap::from([(split_tree.root_node, 1)]);
    let mut heap =
        BinaryHeap::from_iter(split_tree.adjacency_list.keys().map(|node| Reverse(*node)));

    while let Some(Reverse(next_node)) = heap.pop() {
        if let Some(parents) = inverted_adjacency_list.get(&next_node) {
            path_counts.insert(
                next_node,
                parents
                    .iter()
                    .map(|parent| path_counts.get(parent).unwrap())
                    .sum(),
            );
        }
    }

    path_counts
}

fn main() {
    let input = include_str!("../inputs/data_day_7.txt");
    let (start_index, map) = parse_input(input);
    let split_tree = construct_beam_split_tree(start_index, &map);

    // Solution for puzzle 1
    let split_amount = split_tree.adjacency_list.keys().len();
    println!("The tachyon beam is split {split_amount} times");

    // Solution for puzzle 2
    let beam_ends = find_beam_ends(&split_tree.adjacency_list);
    let paths = count_paths(&split_tree);
    let paths_counts = beam_ends
        .iter()
        .map(|(node, ends)| paths.get(node).unwrap() * ends)
        .sum::<u64>();
    println!("A tachyon beam particle could end in {paths_counts} timelines");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "
            ..S..
            .....
            ..^..
            .....
            .^.^.
        ";

        assert_eq!(
            parse_input(input),
            (
                2,
                vec![
                    vec!['.', '.', 'S', '.', '.'],
                    vec!['.', '.', '.', '.', '.'],
                    vec!['.', '.', '^', '.', '.'],
                    vec!['.', '.', '.', '.', '.'],
                    vec!['.', '^', '.', '^', '.'],
                ]
            )
        );
    }

    #[test]
    fn test_construct_beam_split_tree() {
        let map = vec![
            vec!['.', '.', '.', 'S', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '^', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '^', '.', '^', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '^', '.', '^', '.', '.', '^'],
            vec!['.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '^', '.', '^', '^', '.'],
        ];
        let Tree {
            root_node,
            adjacency_list,
        } = construct_beam_split_tree(3, &map);

        assert_eq!(root_node, (2, 3));
        assert_eq!(
            adjacency_list,
            HashMap::from([
                ((2, 3), (Some((4, 2)), Some((4, 4)))),
                ((4, 2), (Some((6, 1)), Some((6, 3)))),
                ((4, 4), (Some((6, 3)), Some((8, 5)))),
                ((6, 1), (None, Some((8, 2)))),
                ((6, 3), (Some((8, 2)), Some((8, 4)))),
                ((8, 2), (None, None)),
                ((8, 4), (None, None)),
                ((8, 5), (None, None))
            ])
        );
    }

    #[test]
    fn test_find_beam_ends() {
        let adjacency_list = HashMap::from([
            ((2, 3), (Some((4, 2)), Some((4, 4)))),
            ((4, 2), (Some((6, 1)), Some((6, 3)))),
            ((4, 4), (Some((6, 3)), Some((8, 5)))),
            ((6, 1), (None, Some((8, 2)))),
            ((6, 3), (Some((8, 2)), Some((8, 4)))),
            ((8, 2), (None, None)),
            ((8, 4), (None, None)),
            ((8, 5), (None, None)),
        ]);

        assert_eq!(
            find_beam_ends(&adjacency_list),
            HashSet::from([((6, 1), 1), ((8, 2), 2), ((8, 4), 2), ((8, 5), 2)])
        );
    }

    #[test]
    fn test_invert_adjacency_list() {
        let adjacency_list = HashMap::from([
            ((2, 3), (Some((4, 2)), Some((4, 4)))),
            ((4, 2), (Some((6, 1)), Some((6, 3)))),
            ((4, 4), (Some((6, 3)), Some((8, 5)))),
            ((6, 1), (None, Some((8, 2)))),
            ((6, 3), (Some((8, 2)), Some((8, 4)))),
            ((8, 2), (None, None)),
            ((8, 4), (None, None)),
            ((8, 5), (None, None)),
        ]);

        assert_eq!(
            invert_adjacency_list(&adjacency_list),
            HashMap::from([
                ((8, 2), HashSet::from([(6, 1), (6, 3)])),
                ((8, 4), HashSet::from([(6, 3)])),
                ((8, 5), HashSet::from([(4, 4)])),
                ((6, 1), HashSet::from([(4, 2)])),
                ((6, 3), HashSet::from([(4, 2), (4, 4)])),
                ((4, 2), HashSet::from([(2, 3)])),
                ((4, 4), HashSet::from([(2, 3)]))
            ])
        );
    }

    #[test]
    fn test_count_paths() {
        let adjacency_list = HashMap::from([
            ((2, 3), (Some((4, 2)), Some((4, 4)))),
            ((4, 2), (Some((6, 1)), Some((6, 3)))),
            ((4, 4), (Some((6, 3)), Some((8, 5)))),
            ((6, 1), (None, Some((8, 2)))),
            ((6, 3), (Some((8, 2)), Some((8, 4)))),
            ((8, 2), (None, None)),
            ((8, 4), (None, None)),
            ((8, 5), (None, None)),
        ]);
        let split_tree = Tree {
            root_node: (2, 3),
            adjacency_list,
        };

        assert_eq!(
            count_paths(&split_tree),
            HashMap::from([
                ((2, 3), 1),
                ((4, 2), 1),
                ((4, 4), 1),
                ((6, 1), 1),
                ((6, 3), 2),
                ((8, 2), 3),
                ((8, 4), 2),
                ((8, 5), 1),
            ])
        );
    }
}
