use circular_buffer::CircularBuffer;
use tether_agent::{PlugDefinition, TetherAgent};

pub const MONITOR_LOG_LENGTH: usize = 256;

type MessageLogEntry = (String, String);
pub struct Insights {
    topics: Vec<String>,
    roles: Vec<String>,
    ids: Vec<String>,
    message_log: CircularBuffer<MONITOR_LOG_LENGTH, MessageLogEntry>,
    message_count: i64,
    monitor_plug: PlugDefinition,
}

impl Insights {
    pub fn new(agent: &TetherAgent) -> Self {
        let monitor_plug = agent
            .create_input_plug("monitor", None, Some("#"))
            .expect("failed to create monitor Input Plug");

        Insights {
            topics: Vec::new(),
            roles: Vec::new(),
            ids: Vec::new(),
            message_log: CircularBuffer::new(),
            message_count: 0,
            monitor_plug,
        }
    }

    pub fn update(&mut self, agent: &TetherAgent) {
        while let Some((_plug_name, message)) = agent.check_messages() {
            let bytes = message.payload();
            let value: rmpv::Value =
                rmp_serde::from_slice(bytes).expect("failed to decode msgpack");
            let json = serde_json::to_string(&value).expect("failed to stringify JSON");
            self.message_log.push_back((message.topic().into(), json));
            add_if_unique(message.topic(), &mut self.topics);
        }
    }

    pub fn message_log(&self) -> &CircularBuffer<MONITOR_LOG_LENGTH, MessageLogEntry> {
        &self.message_log
    }

    pub fn topics(&self) -> &[String] {
        &self.topics
    }
}

fn add_if_unique(item: &str, list: &mut Vec<String>) {
    if list.iter().find(|&i| i.eq(item)).is_none() {
        list.push(String::from(item));
    }
}
