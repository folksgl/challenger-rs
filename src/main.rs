mod gamestate;
mod position;
mod uci;

#[macro_use]
extern crate lazy_static;

fn main() {
    uci::start_uci_engine();
}
