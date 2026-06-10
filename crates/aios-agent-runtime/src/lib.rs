//! Agent Runtime: manages agents as system processes.

use aios_types::{AgentProcess, AgentProcessState, Task};

#[derive(Debug, Default)]
pub struct AgentRuntime;

impl AgentRuntime {
    pub fn spawn_for_task(&self, task: &Task, agent_type: impl Into<String>) -> AgentProcess {
        AgentProcess {
            id: format!("agent_for_{}", task.id),
            task_id: task.id.clone(),
            agent_type: agent_type.into(),
            state: AgentProcessState::Created,
        }
    }

    pub fn start(&self, process: &mut AgentProcess) {
        process.state = AgentProcessState::Running;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aios_types::TaskState;

    #[test]
    fn starts_agent_process() {
        let runtime = AgentRuntime;
        let task = Task {
            id: "task_1".to_string(),
            goal_id: "goal_1".to_string(),
            title: "select application".to_string(),
            state: TaskState::Pending,
        };
        let mut process = runtime.spawn_for_task(&task, "app_invocation_agent");

        runtime.start(&mut process);

        assert_eq!(process.state, AgentProcessState::Running);
    }
}

