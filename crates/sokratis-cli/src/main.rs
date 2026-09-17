//! ZAŠTO RUST OVAKO (cigla M1/1 — kostur CLI)
//! Binarna je tanka: `clap` parsira argumente, `anyhow` nosi grešku do `main`, a `main` je jedino
//! mjesto koje zove `std::process::exit` — izlazni kod je ugovor prema preflightu (0/1/2/3).
mod table;
fn main() {
    eprintln!("sokratis: kostur (cigla M1/18 puni naredbe)");
    std::process::exit(3);
}
