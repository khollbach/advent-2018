use std::{io, iter};
use std::io::BufRead;
use itertools::Itertools;

fn read_input(input: impl BufRead) -> Vec<usize> {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    line.split_whitespace().map(|n| {
        n.parse().unwrap()
    }).collect()
}

pub fn main() {
    let nums = read_input(io::stdin().lock());

    let tree = parse_tree(&mut nums.into_iter());
    println!("{}", tree.metadata_sum());
    println!("{}", tree.value());
}

struct Tree {
    children: Vec<Tree>,
    metadata: Vec<usize>,
}

impl Tree {
    fn metadata_sum(&self) -> usize {
        let self_sum: usize = self.metadata.iter().sum();
        let children_sum: usize = self.children.iter().map(Tree::metadata_sum).sum();
        self_sum + children_sum
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata_sum()
        } else {
            let child_values: Vec<_> = self.children.iter().map(Tree::value).collect();

            self.metadata.iter().filter_map(|&m| {
                if 1 <= m && m <= child_values.len() {
                    Some(child_values[m - 1])
                } else {
                    None
                }
            }).sum()
        }
    }
}

fn parse_tree(nums: &mut impl Iterator<Item=usize>) -> Tree {
    let num_children = nums.next().unwrap();
    let num_metadata = nums.next().unwrap();

    let children = iter::repeat_with(|| {
        parse_tree(nums)
    }).take(num_children).collect();

    let metadata = nums.take(num_metadata).collect();

    Tree { children, metadata }
}
