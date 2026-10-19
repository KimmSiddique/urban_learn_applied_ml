use super::agent_attributes::{AgentAttributes, AgentMode};

pub(crate) struct Agent {
    pub(crate) attributes: AgentAttributes,
}

impl Agent {
    pub(crate) fn new(agent_mode: AgentMode, agent_energy: u32) -> Self {
        Self {
            attributes: AgentAttributes::new(agent_mode, agent_energy),
        }
    }
}
