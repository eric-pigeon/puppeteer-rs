// This file is auto-generated do not edit manually.

// Run-time execution metric.
pub struct Metric {
    // Metric name.
    pub name: String,
    // Metric value.
    pub value: f64,
}

// Disable collecting and reporting metrics.
pub struct Disable {}
pub struct DisableReturnObject {}
pub enum TimeDomain {
    TimeTicks,
    ThreadTicks,
}
// Enable collecting and reporting metrics.
pub struct Enable {
    // Time domain to use for collecting and reporting duration metrics.
    pub time_domain: Option<TimeDomain>,
}
pub struct EnableReturnObject {}
// Sets time domain to use for collecting and reporting duration metrics.
// Note that this must be called before enabling metrics collection. Calling
// this method while metrics collection is enabled returns an error.
pub struct SetTimeDomain {
    // Time domain
    pub time_domain: TimeDomain,
}
pub struct SetTimeDomainReturnObject {}
// Retrieve current values of run-time metrics.
pub struct GetMetrics {}
pub struct GetMetricsReturnObject {
    // Current values for run-time metrics.
    pub metrics: Vec<Metric>,
}
