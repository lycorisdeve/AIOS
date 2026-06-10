//! Goal Kernel: turns user goals into task graphs.

use aios_types::{Goal, Task, TaskState};

#[derive(Debug, Default)]
pub struct GoalKernel;

impl GoalKernel {
    pub fn decompose(&self, goal: &Goal) -> Vec<Task> {
        let task_titles = [
            "understand intent",
            "select application",
            "prepare preview",
            "wait for confirmation",
        ];

        task_titles
            .iter()
            .enumerate()
            .map(|(index, title)| Task {
                id: format!("{}_task_{}", goal.id, index + 1),
                goal_id: goal.id.clone(),
                title: (*title).to_string(),
                state: TaskState::Pending,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_task_graph() {
        let kernel = GoalKernel;
        let goal = Goal::new("goal_1", "Tell Zhang San dinner is waxiang chicken", "user_1");
        let tasks = kernel.decompose(&goal);

        assert_eq!(tasks.len(), 4);
        assert_eq!(tasks[0].goal_id, "goal_1");
    }
}

