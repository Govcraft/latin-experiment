//! SensorActor: wraps a Sensor for concurrent measurement via acton-reactive.

use std::sync::Arc;

use acton_reactive::prelude::*;

use crate::messages::{MeasureRegion, MeasurementResult};
use crate::pressure::Sensor;

/// Actor state for SensorActor.
#[derive(Default, Clone)]
pub struct SensorActorState {
    /// The wrapped sensor implementation
    sensor: Option<Arc<dyn Sensor>>,
    /// Handle to coordinator for sending results back
    coordinator: Option<ActorHandle>,
}

impl std::fmt::Debug for SensorActorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SensorActorState")
            .field("sensor", &self.sensor.as_ref().map(|s| s.name()))
            .field("coordinator", &self.coordinator.is_some())
            .finish()
    }
}

/// Actor wrapper for a [`Sensor`] implementation.
///
/// Each SensorActor handles `MeasureRegion` messages concurrently (via `act_on`)
/// and sends `MeasurementResult` back to the coordinator with the correlation ID.
pub struct SensorActor {
    /// The wrapped sensor implementation
    pub sensor: Arc<dyn Sensor>,
    /// Handle to coordinator for sending results back
    pub coordinator: ActorHandle,
}

impl SensorActor {
    /// Create a new SensorActor.
    pub fn new(sensor: Arc<dyn Sensor>, coordinator: ActorHandle) -> Self {
        Self { sensor, coordinator }
    }

    /// Spawn this sensor actor in the given runtime.
    pub async fn spawn(self, runtime: &mut ActorRuntime) -> ActorHandle {
        let sensor_name = self.sensor.name().to_string();

        let mut actor = runtime.new_actor_with_name::<SensorActorState>(
            format!("Sensor:{}", sensor_name),
        );

        // Set initial state
        actor.model.sensor = Some(self.sensor);
        actor.model.coordinator = Some(self.coordinator);

        // act_on = concurrent (multiple measurements in parallel)
        actor.act_on::<MeasureRegion>(|actor, context| {
            let msg = context.message().clone();
            let sensor = actor.model.sensor.clone();
            let coordinator = actor.model.coordinator.clone();

            let Some(sensor) = sensor else {
                tracing::error!("SensorActor: sensor not initialized");
                return Reply::ready();
            };

            let Some(coordinator) = coordinator else {
                tracing::error!("SensorActor: coordinator not initialized");
                return Reply::ready();
            };

            let sensor_name = sensor.name().to_string();

            // Measure synchronously
            let result = sensor.measure(&msg.region_view);

            match result {
                Ok(signals) => {
                    let measurement = MeasurementResult {
                        correlation_id: msg.correlation_id,
                        region_id: msg.region_id,
                        sensor_name,
                        signals,
                    };

                    // Send result back to coordinator
                    Reply::pending(async move {
                        coordinator.send(measurement).await;
                    })
                }
                Err(e) => {
                    tracing::warn!(
                        sensor = sensor_name,
                        region = %msg.region_id,
                        error = %e,
                        "Sensor measurement failed"
                    );
                    Reply::ready()
                }
            }
        });

        actor.start().await
    }
}
