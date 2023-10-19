use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("CoinFlip", "abis/CoinFlip.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("src/abi/CoinFlip.rs")
}
