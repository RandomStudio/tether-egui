use circular_buffer::CircularBuffer;
use tether_agent::{mqtt::Message, parse_agent_id, parse_agent_role, parse_plug_name, TetherAgent};

pub const MONITOR_LOG_LENGTH: usize = 256;

type MessageLogEntry = (String, String);
pub struct Insights {
    topics: Vec<String>,
    roles: Vec<String>,
    ids: Vec<String>,
    plugs: Vec<String>,
    message_log: CircularBuffer<MONITOR_LOG_LENGTH, MessageLogEntry>,
    message_count: u64,
}

impl Insights {
    /// Create one plug that subscribes to all channels defined by topic; typically `#`
    pub fn new(agent: &TetherAgent, topic: &str) -> Self {
        if agent.is_connected() {
            let _monitor_plug = agent
                .create_input_plug("monitor", None, Some(topic))
                .expect("failed to create monitor Input Plug");
        }

        Insights {
            topics: Vec::new(),
            roles: Vec::new(),
            ids: Vec::new(),
            plugs: Vec::new(),
            message_log: CircularBuffer::new(),
            message_count: 0,
        }
    }

    pub fn update(&mut self, _plug_name: &str, message: &Message) {
        self.message_count += 1;
        let bytes = message.payload();
        if bytes.is_empty() {
            self.message_log
                .push_back((message.topic().into(), "[EMPTY_MESSAGE]".into()));
        } else {
            let value: rmpv::Value =
                rmp_serde::from_slice(bytes).expect("failed to decode msgpack");
            let json = serde_json::to_string(&value).expect("failed to stringify JSON");
            self.message_log.push_back((message.topic().into(), json));
        }

        // Collect some stats...
        add_if_unique(message.topic(), &mut self.topics);
        add_if_unique(
            parse_agent_role(message.topic()).unwrap_or("unknown"),
            &mut self.roles,
        );
        add_if_unique(
            parse_agent_id(message.topic()).unwrap_or("unknown"),
            &mut self.ids,
        );
        add_if_unique(
            parse_plug_name(message.topic()).unwrap_or("unknown"),
            &mut self.plugs,
        );
    }

    pub fn message_log(&self) -> &CircularBuffer<MONITOR_LOG_LENGTH, MessageLogEntry> {
        &self.message_log
    }

    pub fn topics(&self) -> &[String] {
        &self.topics
    }

    pub fn roles(&self) -> &[String] {
        &self.roles
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn plugs(&self) -> &[String] {
        &self.plugs
    }

    pub fn message_count(&self) -> u64 {
        self.message_count
    }
}

fn add_if_unique(item: &str, list: &mut Vec<String>) {
    if !list.iter().any(|i| i.eq(item)) {
        list.push(String::from(item));
    }
}
