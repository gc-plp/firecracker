// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;
use std::sync::{Arc, Mutex};

pub use devices::virtio::balloon::device::BalloonStats;
use devices::virtio::Balloon;
pub use devices::virtio::BALLOON_DEV_ID;

use serde::{Deserialize, Serialize};

type MutexBalloon = Arc<Mutex<Balloon>>;

/// Errors associated with the operations allowed on the balloon.
#[derive(Debug)]
pub enum BalloonConfigError {
    /// The user made a request on an inexistent balloon device.
    DeviceNotFound,
    /// The user polled the statistics of a balloon device that
    /// does not have the statistics enabled.
    StatsNotFound,
    /// Failed to create a balloon device.
    CreateFailure(devices::virtio::balloon::Error),
    /// Failed to update the configuration of the ballon device.
    UpdateFailure(std::io::Error),
}

impl fmt::Display for BalloonConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        use self::BalloonConfigError::*;
        match self {
            DeviceNotFound => write!(f, "No balloon device found."),
            StatsNotFound => write!(f, "Statistics for the balloon device are not enabled"),
            CreateFailure(e) => write!(f, "Error creating the balloon device: {:?}", e),
            UpdateFailure(e) => write!(
                f,
                "Error updating the balloon device configuration: {:?}",
                e
            ),
        }
    }
}

type Result<T> = std::result::Result<T, BalloonConfigError>;

/// This struct represents the strongly typed equivalent of the json body
/// from balloon related requests.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BalloonDeviceConfig {
    /// Number of pages that the balloon should contain.
    pub num_pages: u32,
    /// Option to make the guest obtain permission from the host in
    /// order to deflate.
    pub must_tell_host: bool,
    /// Option to deflate the balloon in case the guest is out of memory.
    pub deflate_on_oom: bool,
    /// Interval in seconds between refreshing statistics.
    pub stats_polling_interval_s: u16,
}

/// The data fed into a balloon update request. Currently, only the number
/// of pages and the stats polling interval can be updated.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BalloonUpdateConfig {
    /// Number of pages that the balloon should contain.
    pub num_pages: u32,
    /// Interval in seconds between refreshing statistics.
    pub stats_polling_interval_s: u16,
}

/// A builder for `Balloon` devices from 'BalloonDeviceConfig'.
#[derive(Default)]
pub struct BalloonBuilder {
    inner: Option<MutexBalloon>,
}

impl BalloonBuilder {
    /// Creates an empty Balloon Store.
    pub fn new() -> Self {
        Self { inner: None }
    }

    /// Inserts a Balloon device in the store.
    /// If an entry already exists, it will overwrite it.
    pub fn set(&mut self, cfg: BalloonDeviceConfig) -> Result<()> {
        self.inner = Some(Arc::new(Mutex::new(
            Balloon::new(
                cfg.num_pages,
                cfg.must_tell_host,
                cfg.deflate_on_oom,
                cfg.stats_polling_interval_s,
                // `restored` flag is false because this code path
                // is never called by snapshot restore functionality.
                false,
            )
            .map_err(BalloonConfigError::CreateFailure)?,
        )));

        Ok(())
    }

    /// Provides a reference to the Balloon if present.
    pub fn get(&self) -> Option<&MutexBalloon> {
        self.inner.as_ref()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn default_config() -> BalloonDeviceConfig {
        BalloonDeviceConfig {
            num_pages: 0,
            must_tell_host: false,
            deflate_on_oom: false,
            stats_polling_interval_s: 0,
        }
    }

    #[test]
    fn test_balloon_create() {
        let balloon_config = default_config();
        let mut builder = BalloonBuilder::new();
        assert!(builder.get().is_none());

        builder.set(balloon_config).unwrap();
        assert_eq!(builder.get().unwrap().lock().unwrap().num_pages(), 0);
    }

    #[test]
    fn test_error_messages() {
        use super::BalloonConfigError::*;
        use std::io;
        let err = CreateFailure(devices::virtio::balloon::Error::EventFd(
            io::Error::from_raw_os_error(0),
        ));
        let _ = format!("{}{:?}", err, err);

        let err = DeviceNotFound;
        let _ = format!("{}{:?}", err, err);

        let err = StatsNotFound;
        let _ = format!("{}{:?}", err, err);
    }
}
