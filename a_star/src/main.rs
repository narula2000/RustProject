extern crate indexmap;
extern crate itertools;
use indexmap::map::Entry::{Occupied, Vacant};
use indexmap::IndexMap;
use num_traits::Zero;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::ops::Sub;
use std::usize;

fn main() {
    static GOAL: Pos = Pos(4, 7);
    let result = astar(
        &Pos(1, 1),
        |p| p.moves(),
        |p| p.distance(&GOAL) / 3,
        |p| *p == GOAL,
    );

    println!("Passed: {:?}", result.expect("Passs?"));
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

fn abs_sub<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (abs_sub(self.0, other.0) + abs_sub(self.1, other.1)) as u32
    }

    fn moves(&self) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        vec![
            Pos(x + 1, y + 2),
            Pos(x + 1, y - 2),
            Pos(x - 1, y + 2),
            Pos(x - 1, y - 2),
            Pos(x + 2, y + 1),
            Pos(x + 2, y - 1),
            Pos(x - 2, y + 1),
            Pos(x - 2, y - 1),
        ]
        .into_iter()
        .map(|p| (p, 1))
        .collect()
    }
}

struct SmallestCostHolder<K> {
    estimated_cost: K,
    cost: K,
    index: usize,
}

impl<K: PartialEq> Eq for SmallestCostHolder<K> {}

impl<K: PartialEq> PartialEq for SmallestCostHolder<K> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl<K: Ord> PartialOrd for SmallestCostHolder<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> Ord for SmallestCostHolder<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            std::cmp::Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

fn reverse_path<N, V, F>(parents: &IndexMap<N, V>, mut parent: F, start: usize) -> Vec<N>
where
    N: Eq + Hash + Clone,
    F: FnMut(&V) -> usize,
{
    let path = itertools::unfold(start, |i| {
        parents.get_index(*i).map(|(node, value)| {
            *i = parent(value);
            node
        })
    })
    .collect::<Vec<&N>>();

    path.into_iter().rev().cloned().collect()
}

fn astar<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestCostHolder {
        estimated_cost: heuristic(start),
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: IndexMap<N, (usize, C)> = IndexMap::new();
    parents.insert(start.clone(), (usize::max_value(), Zero::zero()));
    while let Some(SmallestCostHolder { cost, index, .. }) = to_see.pop() {
        let moves = {
            let (node, &(_, c)) = parents.get_index(index).unwrap();
            if success(node) {
                let path = reverse_path(&parents, |&(p, _)| p, index);
                return Some((path, cost));
            }
            if cost > c {
                continue;
            }
            successors(node)
        };
        for (_move, move_cost) in moves {
            let new_cost = cost + move_cost;
            let h_cost; // heuristic(&successor)
            let idx; // index for successor
            match parents.entry(_move) {
                Vacant(map) => {
                    h_cost = heuristic(map.key());
                    idx = map.index();
                    map.insert((index, new_cost));
                }
                Occupied(mut map) => {
                    if map.get().1 > new_cost {
                        h_cost = heuristic(map.key());
                        idx = map.index();
                        map.insert((index, new_cost));
                    } else {
                        continue;
                    }
                }
            }
            to_see.push(SmallestCostHolder {
                estimated_cost: new_cost + h_cost,
                cost: new_cost,
                index: idx,
            });
        }
    }
    None
}
