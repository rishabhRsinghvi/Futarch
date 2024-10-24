use near_sdk::near_bindgen;
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(Default)]
pub struct RefundContract {
    projects: UnorderedMap<u64, Project>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Project {
    pub id: u64,
    pub total_funds: u128,
    pub milestones: Vec<Milestone>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Milestone {
    pub completed: bool,
}

#[near_bindgen]
impl RefundContract {
    pub fn process_refund(&mut self, project_id: u64) {
        let project = self.projects.get(&project_id).expect("Project not found");
        if project.milestones.iter().any(|m| !m.completed) {
        }
    }
}
