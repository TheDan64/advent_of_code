#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn get_child_node_width(nums: &[usize]) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let num_children = nums[0];
    let num_metadata = nums[1];
    let metadata_start = nums.len() - num_metadata;

    if num_children == 0 {
        2 + num_metadata
    } else {
        let mut total_width = 0;

        for _ in 0..num_children {
            let children_slice = &nums[total_width + 2..metadata_start];
            let width = get_child_node_width(children_slice);

            total_width += width;
        }

        total_width + num_metadata + 2
    }
}

fn build_tree(nums: &[usize]) -> Option<Node> {
    if nums.is_empty() {
        return None;
    }

    let num_children = nums[0];
    let num_metadata = nums[1];
    let mut node = Node {
        children: Vec::with_capacity(num_children),
        metadata: Vec::with_capacity(num_metadata),
    };

    let metadata_start = nums.len() - num_metadata;

    for metadata in &nums[metadata_start..] {
        node.metadata.push(*metadata);
    }

    let children_slice = &nums[2..metadata_start];

    if num_children == 0 {
        return Some(node);
    }

    let num_items = children_slice.len() / num_children;
    let mut start_index = 0;

    for _ in 0..=num_items {
        let width = get_child_node_width(&children_slice[start_index..]);

        if let Some(child) = build_tree(&children_slice[start_index..start_index + width]) {
            node.children.push(child);
        }

        start_index += width;
    }

    Some(node)
}

fn visit_nodes<M, R>(node: &Node, map: &M) -> Vec<R>
where
    M: Fn(&Node) -> R,
{
    let x = map(&node);
    let mut res = vec![x];

    for child in &node.children {
        res.append(&mut visit_nodes(&child, map));
    }

    res
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Node {
    let nums: Vec<usize> = input.split(' ').map(|s| s.parse::<usize>().unwrap()).collect();

    build_tree(&nums).expect("Expected to find a tree")
}

fn sum_metadata(node: &Node) -> usize {
    node.metadata.iter().map(|x| *x).sum()
}

#[aoc(day8, part1, Chars)]
pub fn part1_chars(tree: &Node) -> usize {
    visit_nodes(&tree, &sum_metadata).iter().sum()
}

fn value_node(node: &Node) -> usize {
    if node.children.is_empty() {
        sum_metadata(node)
    } else {
        let mut value = 0;

        for metadata in &node.metadata {
            let index = metadata - 1;
            let child = node.children.get(index);

            if let Some(child) = child {
                let val = value_node(&child);
                value += val;
            }
        }

        value
    }
}

#[aoc(day8, part2, Chars)]
pub fn part2_chars(tree: &Node) -> usize {
    *visit_nodes(&tree, &value_node).iter().next().unwrap()
}
