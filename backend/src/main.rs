//! BillWise backend entry point. The HTTP wiring lands in the next slice;
//! for now `main` is intentionally a stub so `cargo test` exercises the
//! pure-domain unit tests without needing a database.

mod domain;

fn main() {
    println!("billwise: HTTP server not wired yet — run `cargo test` for now.");
}
