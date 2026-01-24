use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use crlf_to_lf_inplace::crlf_to_lf_inplace;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};

fn collect_md_files(dir: &Path, paths: &mut Vec<PathBuf>) {
    for entry in
        fs::read_dir(dir).unwrap_or_else(|err| panic!("failed to read fixtures dir {dir:?}: {err}"))
    {
        let path = entry
            .unwrap_or_else(|err| panic!("failed to read fixtures entry {dir:?}: {err}"))
            .path();
        if path.is_dir() {
            collect_md_files(&path, paths);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            paths.push(path);
        }
    }
}

fn load_fixture_text() -> String {
    let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("benches")
        .join("fixtures")
        .join("opencode-config");
    let mut paths = Vec::new();
    collect_md_files(&fixtures_dir, &mut paths);
    paths.sort();

    let mut combined = String::new();
    for path in paths {
        let contents = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read fixture {path:?}: {err}"));
        combined.push_str(&contents);
    }
    combined
}

fn normalize_to_crlf(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n");
    normalized.replace('\n', "\r\n")
}

fn normalize_to_lf(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn criterion_benchmark_crlf(c: &mut Criterion) {
    let fixtures = load_fixture_text();
    let input = normalize_to_crlf(&fixtures);

    let mut group = c.benchmark_group("crlf_to_lf");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function("crlf_input", |b| {
        b.iter_batched(
            || input.clone(),
            |mut data| {
                crlf_to_lf_inplace(&mut data);
                black_box(data);
            },
            BatchSize::LargeInput,
        );
    });
    group.finish();
}

fn criterion_benchmark_lf(c: &mut Criterion) {
    let fixtures = load_fixture_text();
    let input = normalize_to_lf(&fixtures);

    let mut group = c.benchmark_group("already_lf");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function("lf_input", |b| {
        b.iter_batched(
            || input.clone(),
            |mut data| {
                crlf_to_lf_inplace(&mut data);
                black_box(data);
            },
            BatchSize::LargeInput,
        );
    });
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_crlf, criterion_benchmark_lf
}

criterion_main!(benches);
