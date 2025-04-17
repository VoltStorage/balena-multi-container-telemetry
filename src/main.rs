use crate::collectors::balena_stats_cli_stdout_collector::BalenaStatsCliCollector;
use crate::collectors::balena_stats_collector::BalenaStatsCollector;
use crate::collectors::balena_stats_collector_config::{
    get_collector_config, BalenaStatsCollectorConfig, CollectorType,
};
use crate::collectors::balena_stats_file_collector::BalenaStatsFileCollector;
use crate::exporters::mqtt::export;
use crate::util::config::{build_path, CONFIG_DIR};
use lazy_static::lazy_static;
use log::{error, info, warn};
use log4rs;
use tokio::time::{self, Duration};

mod collectors;
mod domain;
mod exporters;
mod parsers;
mod util;

lazy_static! {
    pub static ref COLLECTOR_CONFIG: BalenaStatsCollectorConfig = get_collector_config();
}

async fn tick() {
    info!("Starting tick.");

    let collector: Box<dyn BalenaStatsCollector> = match (COLLECTOR_CONFIG).mode {
        CollectorType::CLI => Box::new(BalenaStatsCliCollector),
        CollectorType::FILE => Box::new(BalenaStatsFileCollector),
    };

    match collector.collect() {
        Ok(collection) => {
            info!("Successfully collected stats.");
            export(collection)
        }
        Err(err) => error!("Could not collect stats!: {}", err),
    };

    info!("Ending tick.");
}

#[tokio::main]
async fn main() {
    log4rs::init_file(build_path(vec![&CONFIG_DIR, "log4rs.yaml"]), Default::default()).unwrap();
    warn!("Logging < warn to file only; please see log directory.");

    let mut interval = time::interval(Duration::from_secs(
        COLLECTOR_CONFIG.collection_interval_in_seconds,
    ));

    loop {
        interval.tick().await;
        tick().await;
    }
}
