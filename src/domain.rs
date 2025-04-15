use byte_unit::Byte;

#[derive(Debug, PartialEq)]
pub struct ContainerStats {
    pub(crate) container_id: String,
    pub(crate) container_id_short: String,
    pub(crate) container_name: String,
    pub(crate) service_name: String,
    pub(crate) cpu_usage_in_percent: Option<f32>,
    pub(crate) mem_usage_in_percent: Option<f32>,
    pub(crate) mem_usage: Option<Byte>,
    pub(crate) mem_limit: Option<Byte>,
    pub(crate) network_input: Option<Byte>,
    pub(crate) network_output: Option<Byte>,
    pub(crate) block_device_input: Option<Byte>,
    pub(crate) block_device_output: Option<Byte>,
    pub(crate) amount_of_pids: Option<u16>,
}
