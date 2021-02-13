// This file is auto-generated do not edit manually.
use serde::{Deserialize, Serialize};

// Run-time execution metric.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    // Metric name.
    pub name: String,
    // Metric value.
    pub value: f64,
}

// Disable collecting and reporting metrics.
#[derive(Serialize, Debug)]
pub struct Disable {}
#[derive(Deserialize, Debug, Clone)]
pub struct DisableReturnObject {}
impl super::Command for Disable {
    const NAME: &'static str = "Performance.disable";

    type ReturnObject = DisableReturnObject;
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TimeDomain {
    TimeTicks,
    ThreadTicks,
}
// Enable collecting and reporting metrics.
#[derive(Serialize, Debug)]
pub struct Enable {
    // Time domain to use for collecting and reporting duration metrics.
    pub time_domain: Option<TimeDomain>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct EnableReturnObject {}
impl super::Command for Enable {
    const NAME: &'static str = "Performance.enable";

    type ReturnObject = EnableReturnObject;
}
// Sets time domain to use for collecting and reporting duration metrics.
// Note that this must be called before enabling metrics collection. Calling
// this method while metrics collection is enabled returns an error.
#[derive(Serialize, Debug)]
pub struct SetTimeDomain {
    // Time domain
    pub time_domain: TimeDomain,
}
#[derive(Deserialize, Debug, Clone)]
pub struct SetTimeDomainReturnObject {}
impl super::Command for SetTimeDomain {
    const NAME: &'static str = "Performance.setTimeDomain";

    type ReturnObject = SetTimeDomainReturnObject;
}
// Retrieve current values of run-time metrics.
#[derive(Serialize, Debug)]
pub struct GetMetrics {}
#[derive(Deserialize, Debug, Clone)]
pub struct GetMetricsReturnObject {
    // Current values for run-time metrics.
    pub metrics: Vec<Metric>,
}
impl super::Command for GetMetrics {
    const NAME: &'static str = "Performance.getMetrics";

    type ReturnObject = GetMetricsReturnObject;
}
