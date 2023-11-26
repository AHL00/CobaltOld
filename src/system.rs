use std::time::{Duration, Instant};

use crate::App;

#[derive(PartialEq)]
pub enum EventCallbackType {
    WindowResize,
    ShutDown,
}

pub(crate) enum SystemType {
    Update,
    Startup,
    Timed(Duration),
    EventCallback(EventCallbackType),
}

pub struct System {
    pub name: String,
    pub update: Box<dyn FnMut(&mut App, &Duration)>,
    pub(crate) uuid: uuid::Uuid,
    pub(crate) system_type: SystemType,
    /// The first time, the delta will be off
    /// So before App.run(), we need to set this to the current time
    pub(crate) last_run: Instant,

    pub(crate) scene: Option<String>,
}

impl System {
    pub fn startup<T, S>(name: S, run: T) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(run),
            system_type: SystemType::Startup,
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
            scene: None,
        }
    }

    pub fn event_callback<T, S>(name: S, run: T, event_type: EventCallbackType) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(run),
            system_type: SystemType::EventCallback(event_type),
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
            scene: None
        }
    }

    pub fn timed<T, S>(name: S, scene: Option<impl Into<String>>, run: T, duration: Duration) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(run),
            system_type: SystemType::Timed(duration),
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
            scene: scene.map(|s| s.into())
        }
    }

    pub fn update<T, S>(name: S, scene: Option<impl Into<String>>, run: T) -> System 
    where T: FnMut(&mut App, &Duration) + 'static, S: Into<String>
    {
        System {
            name: name.into(),
            update: Box::new(run),
            system_type: SystemType::Update,
            uuid: uuid::Uuid::new_v4(),
            last_run: Instant::now(),
            scene: scene.map(|s| s.into())
        }
    }
}