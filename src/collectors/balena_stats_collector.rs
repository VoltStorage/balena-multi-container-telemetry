use crate::domain::ContainerStats;

pub trait BalenaStatsCollector {
    fn collect(&self) -> anyhow::Result<Vec<ContainerStats>>;
}