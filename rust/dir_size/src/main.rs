use dir_size::dir_size;

fn main() {
    let size = dir_size("/home/derdilla/Coding/rust");

    let gigabyte = size / 1_000_000_000;
    let size = size % 1_000_000_000;
    let megabyte = size / 1_000_000;
    let size = size % 1_000_000;
    let kilobyte = size / 1_000;
    let bytes = size % 1_000;

    println!("{gigabyte}GB {megabyte}mb {kilobyte}kb {bytes}b");
}
