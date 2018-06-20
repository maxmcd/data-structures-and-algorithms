#[macro_use]
extern crate criterion;
extern crate sorting;

use criterion::Criterion;

macro_rules! gen_test {
    ($lib:ident, $name:tt) => {
        fn $lib(c: &mut Criterion) {
            c.bench_function($name, |b| {
                b.iter(|| {
                    let mut list = sorting::common::random_array(1000);
                    sorting::$lib::sort(&mut list);
                })
            });
        }
    };
}

gen_test!(insertion, "Insertion");
gen_test!(selection, "Selection");
gen_test!(bubble, "Bubble\t");
gen_test!(merge, "Merge\t");
gen_test!(heap, "Heap\t");
gen_test!(quick, "Quick\t");

criterion_group!(benches, insertion, selection, bubble, merge, heap, quick);
criterion_main!(benches);
