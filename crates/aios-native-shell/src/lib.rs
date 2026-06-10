//! Native shell model for AIOS.
//!
//! The final shell should be rendered by native platform backends, not HTML.
//! This crate models the shell state that those renderers will display.

use aios_platform::NativeAppTarget;
use aios_types::Goal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellSurface {
    IntentInput,
    AppPreview(AppPreview),
    Confirmation(ConfirmationRequest),
    SystemStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppPreview {
    pub app_id: String,
    pub title: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationRequest {
    pub title: String,
    pub body: String,
    pub target: NativeAppTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellSession {
    pub active_goal: Goal,
    pub surfaces: Vec<ShellSurface>,
}

impl ShellSession {
    pub fn new(active_goal: Goal) -> Self {
        Self {
            active_goal,
            surfaces: vec![ShellSurface::IntentInput],
        }
    }

    pub fn show_app_preview(&mut self, preview: AppPreview) {
        self.surfaces.push(ShellSurface::AppPreview(preview));
    }

    pub fn request_confirmation(&mut self, request: ConfirmationRequest) {
        self.surfaces.push(ShellSurface::Confirmation(request));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_session_starts_with_intent_input() {
        let goal = Goal::new("goal_1", "Tell Zhang San dinner is waxiang chicken", "user_1");
        let session = ShellSession::new(goal);

        assert_eq!(session.surfaces, vec![ShellSurface::IntentInput]);
    }
}

