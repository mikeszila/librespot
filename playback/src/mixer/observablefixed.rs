use super::{Mixer, MixerConfig, VolumeGetter};
use librespot_core::Error;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Clone)]
pub struct ObservableFixedMixer {
    requested_volume: Arc<AtomicU16>,
}

impl ObservableFixedMixer {
    pub const NAME: &'static str = "observablefixed";
}

impl Mixer for ObservableFixedMixer {
    fn open(_config: MixerConfig) -> Result<Self, Error> {
        Ok(Self {
            requested_volume: Arc::new(AtomicU16::new(u16::MAX)),
        })
    }

    fn volume(&self) -> u16 {
        self.requested_volume.load(Ordering::Relaxed)
    }

    fn set_volume(&self, volume: u16) {
        self.requested_volume.store(volume, Ordering::Relaxed);
        log::debug!("observablefixed requested volume={}", volume);
    }

    fn get_soft_volume(&self) -> Box<dyn VolumeGetter + Send> {
        Box::new(FixedFullScaleVolume)
    }
}

struct FixedFullScaleVolume;

impl VolumeGetter for FixedFullScaleVolume {
    fn attenuation_factor(&self) -> f64 {
        1.0
    }
}
