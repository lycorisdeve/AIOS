//! Shared AIOS system object types.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoalState {
    Draft,
    Active,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub intent: String,
    pub owner: String,
    pub state: GoalState,
}

impl Goal {
    pub fn new(id: impl Into<String>, title: impl Into<String>, owner: impl Into<String>) -> Self {
        let title = title.into();

        Self {
            id: id.into(),
            intent: title.clone(),
            title,
            owner: owner.into(),
            state: GoalState::Active,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub goal_id: String,
    pub title: String,
    pub state: TaskState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    RequiresApproval,
    RequiresVerification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCapability {
    pub name: String,
    pub permission_scope: String,
    pub risk_level: RiskLevel,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentProcessState {
    Created,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentProcess {
    pub id: String,
    pub task_id: String,
    pub agent_type: String,
    pub state: AgentProcessState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn goal_defaults_to_active() {
        let goal = Goal::new("goal_1", "Tell Zhang San dinner is waxiang chicken", "user_1");

        assert_eq!(goal.state, GoalState::Active);
        assert_eq!(goal.intent, goal.title);
    }
}

