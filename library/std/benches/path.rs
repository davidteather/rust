use core::hint::black_box;
use std::collections::{BTreeSet, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::*;

#[bench]
fn bench_path_cmp_fast_path_buf_sort(b: &mut test::Bencher) {
    let prefix = "my/home";
    let mut paths: Vec<_> =
        (0..1000).map(|num| PathBuf::from(prefix).join(format!("file {num}.rs"))).collect();

    paths.sort();

    b.iter(|| {
        black_box(paths.as_mut_slice()).sort_unstable();
    });
}

#[bench]
fn bench_path_cmp_fast_path_long(b: &mut test::Bencher) {
    let prefix = "/my/home/is/my/castle/and/my/castle/has/a/rusty/workbench/";
    let paths: Vec<_> =
        (0..1000).map(|num| PathBuf::from(prefix).join(format!("file {num}.rs"))).collect();

    let mut set = BTreeSet::new();

    paths.iter().for_each(|p| {
        set.insert(p.as_path());
    });

    b.iter(|| {
        set.remove(paths[500].as_path());
        set.insert(paths[500].as_path());
    });
}

#[bench]
fn bench_path_cmp_fast_path_short(b: &mut test::Bencher) {
    let prefix = "my/home";
    let paths: Vec<_> =
        (0..1000).map(|num| PathBuf::from(prefix).join(format!("file {num}.rs"))).collect();

    let mut set = BTreeSet::new();

    paths.iter().for_each(|p| {
        set.insert(p.as_path());
    });

    b.iter(|| {
        set.remove(paths[500].as_path());
        set.insert(paths[500].as_path());
    });
}

#[bench]
fn bench_path_components_iter(b: &mut test::Bencher) {
    let p = Path::new("/my/home/is/my/castle/and/my/castle/has/a/rusty/workbench/");

    b.iter(|| {
        for c in black_box(p).components() {
            black_box(c);
        }
    })
}

#[bench]
fn bench_path_file_name(b: &mut test::Bencher) {
    let p1 = Path::new("foo.bar");
    let p2 = Path::new("foo/bar");
    let p3 = Path::new("/bar");

    b.iter(|| {
        black_box(black_box(p1).file_name());
        black_box(black_box(p2).file_name());
        black_box(black_box(p3).file_name());
    })
}

#[bench]
fn bench_path_hashset(b: &mut test::Bencher) {
    let prefix = "/my/home/is/my/castle/and/my/castle/has/a/rusty/workbench/";
    let paths: Vec<_> =
        (0..1000).map(|num| PathBuf::from(prefix).join(format!("file {num}.rs"))).collect();

    let mut set = HashSet::new();

    paths.iter().for_each(|p| {
        set.insert(p.as_path());
    });

    b.iter(|| {
        set.remove(paths[500].as_path());
        set.insert(black_box(paths[500].as_path()))
    });
}

#[bench]
fn bench_path_hashset_miss(b: &mut test::Bencher) {
    let prefix = "/my/home/is/my/castle/and/my/castle/has/a/rusty/workbench/";
    let paths: Vec<_> =
        (0..1000).map(|num| PathBuf::from(prefix).join(format!("file {num}.rs"))).collect();

    let mut set = HashSet::new();

    paths.iter().for_each(|p| {
        set.insert(p.as_path());
    });

    let probe = PathBuf::from(prefix).join("other");

    b.iter(|| set.remove(black_box(probe.as_path())));
}

#[bench]
fn bench_hash_path_short(b: &mut test::Bencher) {
    let mut hasher = DefaultHasher::new();
    let path = Path::new("explorer.exe");

    b.iter(|| black_box(path).hash(&mut hasher));

    black_box(hasher.finish());
}

#[bench]
fn bench_hash_path_long(b: &mut test::Bencher) {
    let mut hasher = DefaultHasher::new();
    let path =
        Path::new("/aaaaa/aaaaaa/./../aaaaaaaa/bbbbbbbbbbbbb/ccccccccccc/ddddddddd/eeeeeee.fff");

    b.iter(|| black_box(path).hash(&mut hasher));

    black_box(hasher.finish());
}

#[bench]
fn bench_path_join_short(b: &mut test::Bencher) {
    let base = Path::new("src");
    let file = Path::new("main.rs");

    b.iter(|| black_box(base).join(black_box(file)));
}

#[bench]
fn bench_path_join_typical(b: &mut test::Bencher) {
    let base = Path::new("/home/user/projects/my-crate/src/some/module");
    let file = Path::new("submodule_file_name.rs");

    b.iter(|| black_box(base).join(black_box(file)));
}

#[bench]
fn bench_path_join_many(b: &mut test::Bencher) {
    let base = Path::new("/home/user/projects/my-crate/target/debug/deps");
    let names: Vec<String> = (0..1000).map(|i| format!("file_{i}.rlib")).collect();

    b.iter(|| {
        for name in &names {
            black_box(black_box(base).join(black_box(name)));
        }
    });
}
