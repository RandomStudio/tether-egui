use tether_agent::TetherAgent;

use crate::{project::EditableTetherSettings, settings::LOCALHOST};

pub fn attempt_new_tether_connection(
    editable_settings: &EditableTetherSettings,
    role: &str,
    id: &str,
) -> Result<TetherAgent, ()> {
    let tether_agent = TetherAgent::new(
        role,
        Some(id),
        Some(editable_settings.host.parse().unwrap_or(LOCALHOST)),
    );
    let username = if editable_settings.username == "" {
        None
    } else {
        Some(editable_settings.username.clone())
    };
    let password = if editable_settings.password == "" {
        None
    } else {
        Some(editable_settings.password.clone())
    };

    match tether_agent.connect(username, password) {
        Ok(()) => Ok(tether_agent),
        Err(_) => Err(()),
    }
}
