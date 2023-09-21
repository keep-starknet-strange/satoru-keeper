#[macro_use]
extern crate log;
use satoru_keeper_core::Keeper;
fn main() {
    env_logger::init();
    info!("starting keeper service...");

    let keeper = Keeper::default();
    keeper.execute_deposit("test");
}
