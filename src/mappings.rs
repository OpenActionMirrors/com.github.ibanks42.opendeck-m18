use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// 18 for M18
// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "18";

// Device layout: 3 rows of 5 LCD keys + 1 row with 3 bottom buttons (+ 2 unused slots)
// OpenDeck sees this as a 4x5 grid (20 slots, but only 18 are real buttons)
pub const ROW_COUNT: usize = 4;
pub const COL_COUNT: usize = 5;
pub const KEY_COUNT: usize = ROW_COUNT * COL_COUNT;
pub const ENCODER_COUNT: usize = 0;

#[derive(Debug, Clone)]
pub enum Kind {
    M18,
}

pub const MIRABOX_VID: u16 = 0x5548;
pub const M18_PID: u16 = 0x1000;

// Map all queries to usage page 65440 and usage id 1
pub const M18_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, MIRABOX_VID, M18_PID);

pub const QUERIES: [DeviceQuery; 1] = [M18_QUERY];

/// Returns correct image format for device kind and key
pub fn get_image_format_for_key(kind: &Kind, _key: u8) -> ImageFormat {
    match kind {
        Kind::M18 => ImageFormat {
            mode: ImageMode::JPEG,
            size: (64, 64),
            rotation: ImageRotation::Rot180,
            mirror: ImageMirroring::Both,
        },
    }
}

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match (vid, pid) {
            (MIRABOX_VID, M18_PID) => Some(Kind::M18),
            _ => None,
        }
    }

    /// Returns protocol version for device
    /// Protocol 1: 512-byte packets, 85x85 images
    /// Protocol 2: 1024-byte packets
    /// Protocol 3: 1024-byte packets, variable image sizes, supports press/release states
    /// M18 uses protocol 3 (supports both press and release states)
    pub fn protocol_version(&self) -> usize {
        match self {
            Self::M18 => 3,
        }
    }

    /// Returns human-readable device name
    pub fn human_name(&self) -> String {
        match self {
            Self::M18 => "VSD Inside M18",
        }
        .to_string()
    }

    /// Because "v1" devices all share the same serial number, use custom suffix to be able to connect
    /// two devices with the different revisions at the same time
    pub fn id_suffix(&self) -> String {
        match self {
            Self::M18 => "M18",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
