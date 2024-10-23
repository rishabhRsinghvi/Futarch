use near_sdk::near_bindgen;
use near_sdk::json_types::U128;
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(Default)]
pub struct FundingContract {
    projects: UnorderedMap<u64, Project>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Project {
    pub id: u64,
    pub owner: String,
    pub total_funds: U128,
    pub milestones: Vec<Milestone>,
    pub is_funded: bool,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Milestone {
    pub description: String,
    pub target_amount: U128,
    pub completed: bool,
}

#[near_bindgen]
impl FundingContract {
    pub fn create_project(&mut self, owner: String) -> u64 {
        let project_id = self.projects.len() as u64;
        self.projects.insert(project_id, Project { id: project_id, owner, total_funds: U128(0), milestones: Vec::new(), is_funded: false });
        project_id
    }

    pub fn fund_project(&mut self, project_id: u64, amount: U128) {
        let mut project = self.projects.get(&project_id).expect("Project not found");
        project.total_funds.0 += amount.0;
        self.projects.insert(project_id, project);
    }

}
