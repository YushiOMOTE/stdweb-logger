use log::*;

fn main() {
    stdweb::initialize();

    stdweb_logger::builder()
        .filter(LevelFilter::Debug)
        .detail()
        .build();

    error!("{} -> {}", "test1", 1);
    warn!("{} -> {}", "test2", 2);
    info!("{} -> {}", "test3", 3);
    debug!("{} -> {}", "test4", 4);
    trace!("{} -> {}", "test5", 5);

    stdweb::event_loop();
}
