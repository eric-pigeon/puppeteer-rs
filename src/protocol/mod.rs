// This file is auto-generated do not edit manually.
use serde;
use serde::{Deserialize, Serialize};

pub mod accessibility;
pub mod animation;
pub mod application_cache;
pub mod audits;
pub mod background_service;
pub mod browser;
pub mod cache_storage;
pub mod cast;
pub mod console;
pub mod css;
pub mod database;
pub mod debugger;
pub mod device_orientation;
pub mod dom;
pub mod dom_debugger;
pub mod emulation;
pub mod fetch;
pub mod headless_experimental;
pub mod heap_profiler;
pub mod indexed_db;
pub mod input;
pub mod inspector;
pub mod io;
pub mod log;
pub mod media;
pub mod memory;
pub mod network;
pub mod overlay;
pub mod page;
pub mod performance;
pub mod profiler;
pub mod runtime;
pub mod schema;
pub mod security;
pub mod service_worker;
pub mod storage;
pub mod system_info;
pub mod target;
pub mod tethering;
pub mod tracing;
pub mod web_audio;
pub mod web_authn;

pub(crate) trait Command {
    const NAME: &'static str;

    type ReturnObject: serde::de::DeserializeOwned;

    fn to_method_call(self, call_id: u32) -> CommandCall<Self>
    where
        Self: std::marker::Sized,
    {
        CommandCall {
            id: call_id,
            params: self,
            method: Self::NAME,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CommandCall<T> {
    method: &'static str,
    pub id: u32,
    params: T,
}
