use std::collections::VecDeque;

use crate::simulation::agent::Agent;

pub(crate) struct AgentManager {
    pub(crate) agents: VecDeque<Agent>,
}

impl AgentManager {
    pub(crate) fn new() -> Self {
        Self {
            agents: VecDeque::new(),
        }
    }
    pub(crate) fn get_agent(&mut self) -> Option<Agent> {
        self.agents.pop_front()
    }
}
