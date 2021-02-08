// This file is auto-generated do not edit manually.

// Describes a single graphics processor (GPU).
pub struct GPUDevice {
    // PCI ID of the GPU vendor, if available; 0 otherwise.
    pub vendor_id: f64,
    // PCI ID of the GPU device, if available; 0 otherwise.
    pub device_id: f64,
    // Sub sys ID of the GPU, only available on Windows.
    pub sub_sys_id: Option<f64>,
    // Revision of the GPU, only available on Windows.
    pub revision: Option<f64>,
    // String description of the GPU vendor, if the PCI ID is not available.
    pub vendor_string: String,
    // String description of the GPU device, if the PCI ID is not available.
    pub device_string: String,
    // String description of the GPU driver vendor.
    pub driver_vendor: String,
    // String description of the GPU driver version.
    pub driver_version: String,
}
// Describes the width and height dimensions of an entity.
pub struct Size {
    // Width in pixels.
    pub width: i32,
    // Height in pixels.
    pub height: i32,
}
// Describes a supported video decoding profile with its associated minimum and
// maximum resolutions.
pub struct VideoDecodeAcceleratorCapability {
    // Video codec profile that is supported, e.g. VP9 Profile 2.
    pub profile: String,
    // Maximum video dimensions in pixels supported for this |profile|.
    pub max_resolution: Size,
    // Minimum video dimensions in pixels supported for this |profile|.
    pub min_resolution: Size,
}
// Describes a supported video encoding profile with its associated maximum
// resolution and maximum framerate.
pub struct VideoEncodeAcceleratorCapability {
    // Video codec profile that is supported, e.g H264 Main.
    pub profile: String,
    // Maximum video dimensions in pixels supported for this |profile|.
    pub max_resolution: Size,
    // Maximum encoding framerate in frames per second supported for this
    // |profile|, as fraction's numerator and denominator, e.g. 24/1 fps,
    // 24000/1001 fps, etc.
    pub max_framerate_numerator: i32,
    pub max_framerate_denominator: i32,
}
// YUV subsampling type of the pixels of a given image.
pub enum SubsamplingFormat {
    Yuv420,
    Yuv422,
    Yuv444,
}
// Image format of a given image.
pub enum ImageType {
    Jpeg,
    Webp,
    Unknown,
}
// Describes a supported image decoding profile with its associated minimum and
// maximum resolutions and subsampling.
pub struct ImageDecodeAcceleratorCapability {
    // Image coded, e.g. Jpeg.
    pub image_type: ImageType,
    // Maximum supported dimensions of the image in pixels.
    pub max_dimensions: Size,
    // Minimum supported dimensions of the image in pixels.
    pub min_dimensions: Size,
    // Optional array of supported subsampling formats, e.g. 4:2:0, if known.
    pub subsamplings: Vec<SubsamplingFormat>,
}
// Provides information about the GPU(s) on the system.
pub struct GPUInfo {
    // The graphics devices on the system. Element 0 is the primary GPU.
    pub devices: Vec<GPUDevice>,
    // An optional dictionary of additional GPU related attributes.
    // TODO objectProperty
    // An optional dictionary of graphics features and their status.
    // TODO objectProperty
    // An optional array of GPU driver bug workarounds.
    pub driver_bug_workarounds: Vec<String>,
    // Supported accelerated video decoding capabilities.
    pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
    // Supported accelerated video encoding capabilities.
    pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
    // Supported accelerated image decoding capabilities.
    pub image_decoding: Vec<ImageDecodeAcceleratorCapability>,
}
// Represents process info.
pub struct ProcessInfo {
    // Specifies process type.
    pub r#type: String,
    // Specifies process id.
    pub id: i32,
    // Specifies cumulative CPU usage in seconds across all threads of the
    // process since the process start.
    pub cpu_time: f64,
}

// Returns information about the system.
pub struct GetInfo {}
pub struct GetInfoReturnObject {
    // Information about the GPUs on the system.
    pub gpu: GPUInfo,
    // A platform-dependent description of the model of the machine. On Mac OS, this is, for
    // example, 'MacBookPro'. Will be the empty string if not supported.
    pub model_name: String,
    // A platform-dependent description of the version of the machine. On Mac OS, this is, for
    // example, '10.1'. Will be the empty string if not supported.
    pub model_version: String,
    // The command line string used to launch the browser. Will be the empty string if not
    // supported.
    pub command_line: String,
}
// Returns information about all running processes.
pub struct GetProcessInfo {}
pub struct GetProcessInfoReturnObject {
    // An array of process info blocks.
    pub process_info: Vec<ProcessInfo>,
}