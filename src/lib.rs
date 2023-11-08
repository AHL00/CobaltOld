use system::System;

pub mod graphics;
pub mod system;

pub struct App {
    
}

pub struct AppBuilder {
    app: Option<App>,
    systems: Vec<System>,

}

impl AppBuilder {

    pub fn new() -> AppBuilder {
        AppBuilder {
            app: None,
            systems: Vec::new(),
        }
    }
  
    pub fn run(mut self) {
        self.build();

        let mut app = self.app.unwrap();

        // Reset the last_run time for all systems
        for system in &mut self.systems {
            system.last_run = std::time::Instant::now();
        }

        loop {
            let systems_ptr: *mut Vec<System> = &mut self.systems;
            for system in unsafe { &mut *systems_ptr } {

                match system.system_type {
                    system::SystemType::Once => {
                        (system.update)(&mut app, &system.last_run.elapsed());
                        self.systems.retain(|s| s.uuid != system.uuid); 
                    },
                    system::SystemType::Timed(duration) => {
                        if system.last_run.elapsed() >= duration {
                            (system.update)(&mut app, &system.last_run.elapsed());
                            system.last_run = std::time::Instant::now();
                        }
                    },
                    system::SystemType::Update => {
                        (system.update)(&mut app, &system.last_run.elapsed());
                        system.last_run = std::time::Instant::now();
                    }
                }
            }
        }
    }

    fn build(&mut self) {
        self.app = Some(App {});
    }

    pub fn register_system(&mut self, system: System) {
        if !self.systems.iter().any(|s| s.uuid == system.uuid) {
            self.systems.push(system);
        }
    }
}