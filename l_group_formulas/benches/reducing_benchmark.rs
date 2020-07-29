use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use l_group_formulas::l_group_term::*;
use l_group_formulas::literal::*;
use std::collections::BTreeSet;
use l_group_formulas::*;
use std::iter;

    fn meet_of_depth(n: usize) -> LGroupTerm {
        let mut current_meetand = LGroupTerm::from(Literal::new('x', 0, false));
        for k in 1 ..= n {
            let new_literal = Literal::new('x', k, false);
            let mut meetands = BTreeSet::new();
            meetands.insert(current_meetand);
            meetands.insert(LGroupTerm::from(new_literal));
            current_meetand = LGroupTerm::Meet(meetands);
        }
        println!("{}", current_meetand.to_string());
        return current_meetand;
    }

    #[allow(dead_code)]
    fn bench_meet_reduce(n: usize) {
        meet_of_depth(n).reduced();
    }

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_meet_reduce");
    for depth in [1000, 2000, 5000, 10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000, 200000, 300000, 500000, 750000, 1000000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(depth), 
            depth,
            |b, &depth| {
                b.iter(|| iter::repeat(0u8).take(depth).collect::<Vec<_>>());
            }
        );

    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
