use std::{borrow::Cow, fs::read_to_string, path::Path};

fn main() {
    let members = std::env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap();
    dbg!(members);
}

fn cached_input(day: u8) -> String {
    let fq_path = format!("{}/input_{}", env!("CARGO_MANIFEST_DIR"), day);
    let path = Path::new(&fq_path);

    if path.exists() {
        return load_file(path.to_string_lossy());
    }

    let input = ureq::get(&format!("https://adventofcode.com/2021/day/{}/input", day))
        .set(
            "Cookie",
            &format!(
                "session={}",
                std::env::var("AOC_2021_SESSION_COOKIE")
                    .expect("AOC_2021_SESSION_COOKIE env var not set")
            ),
        )
        .call()
        .unwrap()
        .into_string()
        .unwrap();

    std::fs::write(&path, input).unwrap();

    load_file(path.to_string_lossy())
}

fn load_file(path: Cow<str>) -> String {
    read_to_string(path.to_string()).unwrap()
}
