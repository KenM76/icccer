//! Prints Pass 5d's whole analysis (README §29). `cargo run --release --bin pass5d_probe`
fn main() {
    let oracle = match icccer_difftest::Oracle::locate() {
        Ok(Some(o)) => o,
        other => {
            eprintln!("no oracle: {other:?}");
            std::process::exit(2);
        }
    };
    match icccer_difftest::pass5d::analyse(&oracle) {
        Ok(a) => println!("{a:#?}"),
        Err(e) => {
            eprintln!("skip: {e}");
            std::process::exit(3);
        }
    }
}
