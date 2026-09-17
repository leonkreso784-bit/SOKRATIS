//! ZAŠTO RUST OVAKO (cigla M1/18 — CLI naredbe i izlazni kodovi)
//! Potpisi za tri prikaza (`render`, `render_docs`, `render_signals`) su ugovor prema `main.rs`
//! već sada, iako je tijelo `todo!()` do cigle M1/19: `main` se može testirati na izlazne kodove
//! bez čekanja na format tablice, jer signatura funkcije (ne implementacija) diktira poziv.
use sokratis_core::{DocsHealth, Report, Signal};
pub fn render(report: &Report) -> String {
    let _ = report;
    todo!("cigla M1/19")
}
pub fn render_docs(docs: &DocsHealth) -> String {
    let _ = docs;
    todo!("cigla M1/19")
}
pub fn render_signals(signals: &[Signal]) -> String {
    let _ = signals;
    todo!("cigla M1/19")
}
