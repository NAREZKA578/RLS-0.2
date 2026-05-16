use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum EngineEvent {
    WindowResized { width: u32, height: u32 },
    StateChanged { from: String, to: String },
    AssetLoaded { path: String },
    Error { message: String },
    Custom { name: String, data: String },
}

impl EngineEvent {
    pub fn variant_name(&self) -> &'static str {
        match self {
            EngineEvent::WindowResized { .. } => "WindowResized",
            EngineEvent::StateChanged { .. } => "StateChanged",
            EngineEvent::AssetLoaded { .. } => "AssetLoaded",
            EngineEvent::Error { .. } => "Error",
            EngineEvent::Custom { .. } => "Custom",
        }
    }
}

pub struct EventBus {
    senders: HashMap<String, Vec<Sender<EngineEvent>>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            senders: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_type: String) -> Receiver<EngineEvent> {
        let (tx, rx) = unbounded::<EngineEvent>();
        self.senders.entry(event_type).or_default().push(tx);
        rx
    }

    pub fn emit(&self, event: &EngineEvent) {
        if let Some(senders) = self.senders.get(event.variant_name()) {
            for tx in senders {
                let _ = tx.send(event.clone());
            }
        }
    }
}
