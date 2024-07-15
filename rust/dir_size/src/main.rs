use dir_size::dir_size;

fn main() {
    let src = std::env::args().nth(1);
    let src = src.unwrap_or(".".to_string());

    let size = dir_size(&src);
    let size = format_bytes(size);

    println!("{src}: {size}");
}

fn format_bytes(size: u128) -> String {
    let gigabyte = size / 1_000_000_000;
    let size = size % 1_000_000_000;
    let megabyte = size / 1_000_000;
    let size = size % 1_000_000;
    let kilobyte = size / 1_000;
    let bytes = size % 1_000;

    format!("{gigabyte}GB {megabyte}mb {kilobyte}kb {bytes}b").to_string()
}
