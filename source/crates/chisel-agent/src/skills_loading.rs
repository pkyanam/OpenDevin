//! Skills loading — discover and load agent skills.
//!
//! Reconstruction skeleton.

/// A discovered skill.
///
/// TODO(reconstruction): the original `chisel-agent/src/skills_loading.rs`
/// discovers skills from the user + project config and the agent-ext skills
/// plugin storage.
#[derive(Debug, Clone, Default)]
pub struct Skill {
    pub name: String,
    pub description: String,
}

pub fn load_skills(_workspace: &std::path::Path) -> Vec<Skill> {
    Vec::new()
}