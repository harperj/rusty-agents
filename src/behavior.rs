use super::communicator_objects as co;
use co::agent_info::AgentInfoProto;
use co::brain_parameters::BrainParametersProto;
use co::space_type::SpaceTypeProto;
use tch::kind::{FLOAT_CPU, INT64_CPU};
use tch::Tensor;
use rand::rngs::ThreadRng;
use rand::Rng;

pub enum ActionSpace {
    Discrete(Vec<i32>),
    Continuous(i32),
}

impl ActionSpace {
    pub fn size(self) -> i32 {
        match self {
            ActionSpace::Discrete(branches) => branches.len() as i32,
            ActionSpace::Continuous(n) => n,
        }
    }

    pub fn random_action(&self, rng: &mut ThreadRng) -> Vec<f32> {
        // let mut rng = rand::thread_rng();
        match self {
            ActionSpace::Discrete(branches) => {
                branches.iter().map(|b| rng.gen_range(0, b) as f32).collect::<Vec<_>>()
            },
            ActionSpace::Continuous(n) => {
                let mut agent_actions = vec![];
                for _ in 0..*n {
                    agent_actions.push(rng.gen_range(-1.0, 1.0));
                }
                agent_actions
            }
        }
    }

    pub fn random_actions(&self, n_agents: usize, rng: &mut ThreadRng) -> Vec<Vec<f32>> {
        let mut all_actions: Vec<Vec<f32>> = vec![];
        for _ in 0..n_agents {
            all_actions.push(self.random_action(rng));
        }
        all_actions
    }
}

pub type ObservationSpace = Vec<Vec<i32>>;
pub struct BehaviorSpec {
    pub obs_space: ObservationSpace,
    pub action_space: ActionSpace,
}

impl BehaviorSpec {
    pub fn from_proto(
        brain_params: &BrainParametersProto,
        agent_info: &AgentInfoProto,
    ) -> BehaviorSpec {
        let observations = &agent_info.observations;
        let mut obs_space: ObservationSpace = vec![];
        for obs in observations.iter() {
            obs_space.push(obs.shape.clone());
        }
        let action_space = match brain_params.vector_action_space_type {
            SpaceTypeProto::discrete => {
                ActionSpace::Discrete(brain_params.vector_action_size.clone())
            }
            SpaceTypeProto::continuous => {
                ActionSpace::Continuous(brain_params.vector_action_size[0])
            }
        };
        BehaviorSpec {
            obs_space,
            action_space,
        }
    }

    pub fn create_empty_action(&self, n_agents: usize) -> Vec<Vec<f32>> {//Tensor {
        match &self.action_space {
            // ActionSpace::Continuous(n) => Tensor::zeros(&[n_agents as i64, *n as i64], FLOAT_CPU),
            // ActionSpace::Discrete(branches) => {
            // Tensor::zeros(&[n_agents as i64, branches.len() as i64], INT64_CPU)
            // }
            ActionSpace::Continuous(n) => vec![vec![0.0; *n as usize]; n_agents],
            ActionSpace::Discrete(branches) => vec![vec![0.0; branches.len()]; n_agents]
        }
    }
}

pub struct DecisionStep {
    pub obs: Vec<Tensor>,
    pub reward: f32,
    pub agent_id: i32,
    pub action_mask: Option<Vec<Tensor>>,
}

pub struct TerminalStep {
    pub obs: Vec<Tensor>,
    pub reward: f32,
    pub agent_id: i32,
    pub interrupted: bool,
}
