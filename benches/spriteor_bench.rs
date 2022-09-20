use criterion::{criterion_group, criterion_main, Criterion};
// use ui_spriteor::{border_box_quarter, border_box_quarter_b, BoxSettings};

fn border_box_benchmark(_c: &mut Criterion) {
  //   let settings = black_box(BoxSettings {
  //     width: 512,
  //     height: 512,
  //     ..Default::default()
  //   });
  //   c.bench_function("mirroring_512by512", |b| {
  //     b.iter(|| border_box_quarter(&settings))
  //   });
  //   c.bench_function("mirroring_b_512by512", |b| {
  //     b.iter(|| border_box_quarter_b(&settings))
  //   });

  //   //128x256
  //   let settings = black_box(BoxSettings {
  //     width: 128,
  //     height: 256,
  //     ..Default::default()
  //   });
  //   c.bench_function("mirroring_128by256", |b| {
  //     b.iter(|| border_box_quarter(&settings))
  //   });
  //   c.bench_function("mirroring_b_128by256", |b| {
  //     b.iter(|| border_box_quarter_b(&settings))
  //   });
}

criterion_group!(benches, border_box_benchmark);
criterion_main!(benches);
