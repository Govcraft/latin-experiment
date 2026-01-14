//! Tick driver actor for receiving TickComplete from the kernel coordinator.
//!
//! This simple actor bridges the kernel's actor-based coordination with the
//! experiment harness by forwarding TickComplete results to an mpsc channel.

use acton_reactive::prelude::*;
use tokio::sync::mpsc;

use survival_kernel::kernel::TickResult;
use survival_kernel::messages::TickComplete;

/// State for the tick driver actor.
#[derive(Default, Clone)]
pub struct TickDriverState {
    /// Channel sender for forwarding tick results
    pub tx: Option<mpsc::Sender<TickResult>>,
}

impl std::fmt::Debug for TickDriverState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TickDriverState")
            .field("has_tx", &self.tx.is_some())
            .finish()
    }
}

/// Actor that receives TickComplete messages and forwards them to a channel.
///
/// The experiment harness creates this actor and registers it with the
/// coordinator via RegisterTickDriver. When the coordinator completes a tick,
/// it sends TickComplete to this actor, which forwards the result to the
/// mpsc channel for the experiment harness to receive.
pub struct TickDriverActor {
    /// Channel sender for forwarding tick results
    tx: mpsc::Sender<TickResult>,
}

impl TickDriverActor {
    /// Create a new tick driver actor with the given channel sender.
    pub fn new(tx: mpsc::Sender<TickResult>) -> Self {
        Self { tx }
    }

    /// Spawn the actor in the runtime.
    ///
    /// Returns the actor handle which should be sent to the coordinator
    /// via RegisterTickDriver.
    pub async fn spawn(self, runtime: &mut ActorRuntime) -> ActorHandle {
        let mut actor = runtime.new_actor_with_name::<TickDriverState>("TickDriver".to_string());

        actor.model.tx = Some(self.tx);

        actor.act_on::<TickComplete>(|actor, context| {
            let result = context.message().result.clone();
            let tx = actor.model.tx.clone();

            Reply::pending(async move {
                if let Some(tx) = tx {
                    // Ignore send errors - receiver may have been dropped
                    let _ = tx.send(result).await;
                }
            })
        });

        actor.start().await
    }
}
