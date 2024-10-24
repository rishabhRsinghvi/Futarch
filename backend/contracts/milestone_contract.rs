use near_sdk::near_bindgen;
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(Default)]
pub struct MilestoneContract {
    projects: UnorderedMap<u64, Project>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Project {
    pub id: u64,
    pub milestones: Vec<Milestone>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Milestone {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

#[near_bindgen]
impl MilestoneContract {
    pub fn add_milestone(&mut self, project_id: u64, description: String) -> u64 {
        let mut project = self.projects.get(&project_id).expect("Project not found");
        let milestone_id = project.milestones.len() as u64;
        project.milestones.push(Milestone { id: milestone_id, description, completed: false });
        self.projects.insert(project_id, project);
        milestone_id
    }

    pub fn complete_milestone(&mut self, project_id: u64, milestone_id: u64) {
        let mut project = self.projects.get(&project_id).expect("Project not found");
        project.milestones[milestone_id as usize].completed = true;
        self.projects.insert(project_id, project);
    }
}
