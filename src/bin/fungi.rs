use game::fungi;

fn main() {
    if let Err(e) = fungi::run(std::env::args()) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}