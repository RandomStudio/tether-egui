use std::{net::IpAddr, time::Duration};

extern crate log;
use log::{debug, error, info};

extern crate paho_mqtt;
use paho_mqtt::{Client, ConnectOptionsBuilder, CreateOptionsBuilder, Message};

extern crate rmp_serde;
extern crate serde;

use rmp_serde::to_vec_named;
use serde::Serialize;

pub struct TetherAgent {
    client: Client,
    role: String,
    id: String,
}

impl TetherAgent {
    pub fn new(broker_host: IpAddr, role: &str, id: Option<&str>) -> Self {
        let broker_uri = format!("tcp://{broker_host}:1883");
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(broker_uri)
            .client_id("")
            .finalize();

        // Create the client connection
        let client = Client::new(create_opts).unwrap();
        TetherAgent {
            client,
            role: String::from(role),
            id: match id {
                Some(s) => String::from(s),
                None => String::from("any"),
            },
        }
    }

    pub fn connect(&self) {
        let conn_opts = ConnectOptionsBuilder::new()
            .user_name("tether")
            .password("sp_ceB0ss!")
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(paho_mqtt::MQTT_VERSION_3_1_1)
            .clean_session(true)
            .finalize();

        // Make the connection to the broker
        debug!("Connecting to the MQTT server...");
        match self.client.connect(conn_opts) {
            Ok(res) => {
                info!("MQTT client connected OK");
                debug!("Connected OK: {res:?}");

                // match self.client.subscribe_many(INPUT_TOPICS, INPUT_QOS) {
                //     Ok(res) => {
                //         debug!("Subscribe OK: {res:?}");
                //     }
                //     Err(e) => {
                //         error!("Error subscribing: {e:?}");
                //     }
                // }
            }
            Err(e) => {
                error!("Error connecting to the broker: {e:?}");
                // process::exit(1);
            }
        }
    }

    pub fn publish<T: Serialize>(&self, data: T) -> Result<(), paho_mqtt::Error> {
        let payload = to_vec_named(&data).unwrap();

        let msg = Message::new("dummy/dummy/test", payload, 1);
        self.client.publish(msg)
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected()
    }

    pub fn set_role(&mut self, role: &str) {
        self.role = String::from(role);
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = String::from(id);
    }
}
