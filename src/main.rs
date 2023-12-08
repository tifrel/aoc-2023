fn main() {
    let input_filename = std::env::args().nth(1).unwrap();
    let input = std::fs::read_to_string(input_filename).expect("File is borked");

    let output = aoc::aoc2023_d01_p2(input);

    println!("{}", output);
}
