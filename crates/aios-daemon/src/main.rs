use aios_agent_runtime::AgentRuntime;
use aios_goal_kernel::GoalKernel;
use aios_policy_kernel::PolicyKernel;
use aios_types::{Goal, RiskLevel, ToolCapability};

fn main() {
    let goal = Goal::new("goal_demo", "Tell Zhang San dinner is waxiang chicken", "user_demo");
    let goal_kernel = GoalKernel;
    let policy_kernel = PolicyKernel;
    let agent_runtime = AgentRuntime;

    let tasks = goal_kernel.decompose(&goal);
    let capability = ToolCapability {
        name: "wechat.send_message".to_string(),
        permission_scope: "chat.write.once".to_string(),
        risk_level: RiskLevel::Low,
        requires_approval: true,
    };

    let decision = policy_kernel.evaluate(&capability);
    let process = agent_runtime.spawn_for_task(&tasks[0], "app_invocation_agent");

    println!("AIOS daemon prototype");
    println!("goal: {}", goal.title);
    println!("tasks: {}", tasks.len());
    println!("policy decision: {:?}", decision);
    println!("agent process: {:?}", process.state);
}

