use criterion::{criterion_group, criterion_main, Criterion};
use raytracer::material::random_point_in_unit_sphere;

pub fn random_unit_sphere_benchmark(c: &mut Criterion) {
    c.bench_function("random_unit_sphere", |b| {
        b.iter(|| random_point_in_unit_sphere())
    });
}

criterion_group!(benches, random_unit_sphere_benchmark);
criterion_main!(benches);
