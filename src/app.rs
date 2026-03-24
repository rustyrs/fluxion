//! Defines the `FluxionApp` struct, which allows users to register systems
//! and run the main tick loop for the server.

use std::time::Duration;

use crate::plugin::*;
use bevy_ecs::{
    error::ErrorHandler, 
    schedule::{IntoScheduleConfigs, Schedule, ScheduleLabel}, 
    system::{Commands, Query, ResMut, ScheduleSystem}, 
    world::World
};

/// A label used to identify the main execution schedule.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MainSchedule;

/// The core application structure that manages the ECS `World` and execution `Schedule`.
/// 
/// It is responsible for registering plugins and systems, and driving the main execution loop.
#[must_use]
pub struct FluxionApp {
    /// The ECS world containing all entities, components, and resources.
    pub world: World,
    /// The main schedule where systems are registered and executed.
    pub schedule: Schedule,
    default_error_handler: Option<ErrorHandler>,
}

impl Default for FluxionApp {
    /// Creates a default instance of `FluxionApp` with an empty `World` and `MainSchedule`.
    fn default() -> Self {
        FluxionApp {
            world: World::new(),
            schedule: Schedule::new(MainSchedule),
            default_error_handler: None,
        }
    }
}

impl FluxionApp {
    /// Creates a new, empty `FluxionApp`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let app = FluxionApp::new();
    /// ```
    pub fn new() -> FluxionApp {
        FluxionApp::default()
    }

    /// Starts the server's main execution loop.
    /// 
    /// This method will continuously run all systems registered in the schedule.
    /// It currently targets a tick rate of 60Hz by sleeping for 16 milliseconds per frame
    /// to prevent maximum CPU utilization.
    /// 
    /// # Note
    /// 
    /// This method contains an infinite loop and will block the current thread indefinitely.
    pub fn run(&mut self) {
        println!("FluxionApp🚀");

        // Server main loop
        loop {
            // Run all systems registered in the schedule
            self.schedule.run(&mut self.world);

            // Prevent CPU maxout, temporarily set to 60Hz
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    /// Returns the current state of the application's plugins.
    /// 
    /// # Panics
    /// 
    /// Currently panics because it is not yet implemented.
    #[inline]
    pub fn plugins_state(&mut self) -> PluginsState {
        todo!();
    }

    /// Adds one or more plugins to the application.
    /// 
    /// # Panics
    /// 
    /// Currently panics because it is not yet implemented.
    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) -> &mut Self {
        todo!();
    }

    /// Adds systems to the application's schedule.
    /// 
    /// # Note
    /// 
    /// Currently, the `_schedule` parameter is ignored, and systems are added
    /// directly to the internal `MainSchedule`.
    pub fn add_systems<M>(
        &mut self,
        _schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        self.schedule.add_systems(systems);
        self
    }
}