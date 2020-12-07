extern crate pathfinding;
use criterion::*;
use std::ops::Sub;
#[path = "../src/main.rs"]
mod main;

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

    // Return the move and it cost (How knight moves)
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

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Astar Seq");
    for i in 2..100 {
        let goal: Pos = Pos(i, i);
        group.bench_with_input(BenchmarkId::new("Mine", i), &i, |b, _i| {
            b.iter(|| {
                main::astar(
                    &Pos(1, 1),
                    |p| p.moves(),
                    |p| p.distance(&goal) / 3,
                    |p| *p == goal,
                )
            })
        });
        group.bench_with_input(BenchmarkId::new("Crate", i), &i, |b, _i| {
            b.iter(|| {
                pathfinding::directed::astar::astar(
                    &Pos(1, 1),
                    |p| p.moves(),
                    |p| p.distance(&goal) / 3,
                    |p| *p == goal,
                )
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
