use aios_goal_kernel::GoalKernel;
use aios_types::Goal;

fn main() {
    let goal_text = std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let goal_text = if goal_text.is_empty() {
        "Tell Zhang San dinner is waxiang chicken".to_string()
    } else {
        goal_text
    };

    let goal = Goal::new("goal_cli", goal_text, "local_user");
    let tasks = GoalKernel.decompose(&goal);

    println!("Goal: {}", goal.title);
    for task in tasks {
        println!("- {}", task.title);
    }
}

