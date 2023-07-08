mod result;

use log::info;
use result::PricerResult;

fn main() -> PricerResult<()> {
    env_logger::init();
    info!("Hello, world!");
    Ok(())
}
