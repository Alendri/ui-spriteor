use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ui_spriteor::{border_box_quarter, border_box_raw, BoxSettings};

fn border_box_benchmark(c: &mut Criterion) {
  c.bench_function("border_box_raw_128by256", |b| {
    let settings = black_box(BoxSettings {
      width: 128,
      height: 256,
      ..Default::default()
    });
    b.iter(|| border_box_raw(black_box(&settings)))
  });
  c.bench_function("mirroring_512by512", |b| {
    let settings = black_box(BoxSettings {
      width: 512,
      height: 512,
      ..Default::default()
    });
    b.iter(|| border_box_quarter(&settings))
  });
  c.bench_function("border_box_raw_512by512", |b| {
    let settings = black_box(BoxSettings {
      width: 512,
      height: 512,
      ..Default::default()
    });
    b.iter(|| border_box_raw(black_box(&settings)))
  });
  c.bench_function("mirroring_128by256", |b| {
    let settings = black_box(BoxSettings {
      width: 128,
      height: 256,
      ..Default::default()
    });
    b.iter(|| border_box_quarter(&settings))
  });
}

criterion_group!(benches, border_box_benchmark);
criterion_main!(benches);
