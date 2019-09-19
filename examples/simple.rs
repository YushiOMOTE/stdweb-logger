use log::*;

fn main() {
    stdweb::initialize();
    stdweb_logger::init();
    info!("{} -> {}", "Chin", "Tama");
    stdweb::event_loop();
}
