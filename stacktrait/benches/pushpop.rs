use stacktrait::*;

use criterion::*;

use std::collections::LinkedList;

fn iterate<'a, S>(mut stack: S)
    where S: Stack<&'a u64>
{
    let iters = 1000000;
    let mut sum = 0;
    let values = &0x1234;
    stack.spush(&values);
    for _ in 0..iters-1 {
        dup(&mut stack);
    }
    while let Some(v) = stack.spop() {
        sum += *v;
    }
    assert_eq!(sum, *values * iters);
}

fn bench(c: &mut Criterion) {
    c.bench_function("vec", |b| b.iter(|| {
        let v = Vec::new();
        iterate(black_box(v))
    }));
    c.bench_function("ll", |b| b.iter(|| {
        let ll = LinkedList::new();
        iterate(black_box(ll))
    }));
}

criterion_group!(benches, bench);
criterion_main!(benches);
