use std::time::{Duration, Instant};

use crate::App;

pub(crate) enum SystemType {
    Update,
    Startup,
    Timed(Duration)
}

pub struct System {
    pub name: String,
    pub update: Box<dyn FnMut(&mut App, &Duration)>,
    pub(crate) uuid: uuid::Uuid,
    pub(crate) system_type: SystemType,
    /// The first time, the delta will be off
    /// So before App.run(), we need to set this to the current time
    pub(crate) last_run: Instant,
}

impl System {
    pub fn startup<T, S>(name: S, update: T) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(update),
            system_type: SystemType::Startup,
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
        }
    }

    pub fn timed<T, S>(name: S, update: T, duration: Duration) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(update),
            system_type: SystemType::Timed(duration),
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
        }
    }

    pub fn update<T, S>(name: S, update: T) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(update),
            system_type: SystemType::Update,
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
        }
    }
}