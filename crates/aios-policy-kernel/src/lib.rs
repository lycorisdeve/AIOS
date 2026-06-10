//! Policy Kernel: controls whether an agent can use a capability.

use aios_types::{PolicyDecision, RiskLevel, ToolCapability};

#[derive(Debug, Default)]
pub struct PolicyKernel;

impl PolicyKernel {
    pub fn evaluate(&self, capability: &ToolCapability) -> PolicyDecision {
        if capability.requires_approval {
            return PolicyDecision::RequiresApproval;
        }

        match capability.risk_level {
            RiskLevel::None | RiskLevel::Low => PolicyDecision::Allow,
            RiskLevel::Medium => PolicyDecision::RequiresVerification,
            RiskLevel::High | RiskLevel::Critical => PolicyDecision::RequiresApproval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_risk_requires_approval() {
        let kernel = PolicyKernel;
        let capability = ToolCapability {
            name: "chat.send".to_string(),
            permission_scope: "chat.write.once".to_string(),
            risk_level: RiskLevel::High,
            requires_approval: false,
        };

        assert_eq!(kernel.evaluate(&capability), PolicyDecision::RequiresApproval);
    }
}

