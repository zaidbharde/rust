fn main() {
    let data = [7u32, 11, 13, 17];
    let checksum: u32 = data.iter().sum::<u32>() % 10;
    println!("checksum={checksum}");
}
