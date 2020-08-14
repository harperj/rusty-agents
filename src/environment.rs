use std::collections::HashMap;
use tch::Tensor;

use super::communicator::*;
use super::communicator_objects::unity_input::UnityInputProto;
use super::communicator_objects::unity_output::UnityOutputProto;
use super::communicator_objects::unity_rl_initialization_input::UnityRLInitializationInputProto;
use super::communicator_objects::unity_rl_input::UnityRLInputProto;
use super::communicator_objects::command::*;
use super::communicator_objects::brain_parameters::*;
use super::communicator_objects::agent_info::AgentInfoProto;
use super::communicator_objects::space_type::SpaceTypeProto;
use super::communicator_objects::capabilities::UnityRLCapabilitiesProto;
use protobuf::SingularPtrField;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("Communicator has exited.")]
    CommunicatorExited
}

pub type EnvironmentResult<T> = Result<T, EnvironmentError>;

pub enum ActionSpace {
    Discrete(Vec<i32>),
    Continuous(i32)
}

impl ActionSpace {
    pub fn size(self) -> i32 {
        match self {
            ActionSpace::Discrete(branches) => branches.len() as i32,
            ActionSpace::Continuous(n) => n
        }
    }
}

pub type ObservationSpace = Vec<Vec<i32>>;
pub struct BehaviorSpec {
    obs_space: ObservationSpace,
    action_space: ActionSpace
}

impl BehaviorSpec {
    pub fn from_proto(brain_params: &BrainParametersProto, agent_info: &AgentInfoProto) -> BehaviorSpec {
        let observations = &agent_info.observations;
        let mut obs_space: ObservationSpace = vec![];
        for obs in observations.iter() {
            obs_space.push(obs.shape.clone());
        }
        let action_space = match brain_params.vector_action_space_type {
            SpaceTypeProto::discrete => {
                ActionSpace::Discrete(brain_params.vector_action_size.clone())
            },
            SpaceTypeProto::continuous => {
                ActionSpace::Continuous(brain_params.vector_action_size[0])
            }
        };
        BehaviorSpec {
            obs_space,
            action_space
        }
    }
}


pub struct DecisionStep {
    obs: Vec<Tensor>,
    reward: f64,
    agent_id: i64,
    action_mask: Option<Vec<Tensor>>
}

pub struct TerminalStep {
    obs: Vec<Tensor>,
    reward: f64,
    agent_id: i64,
    interrupted: bool
}

pub trait Environment {
    fn step(&self) -> EnvironmentResult<()>;
    fn reset(&self) -> EnvironmentResult<()>;
    fn set_actions(&mut self, behavior_name: String, action: Tensor);
    fn get_steps(&self, behavior_name: String) -> (Vec<DecisionStep>, Vec<TerminalStep>);
}

const DEFAULT_EDITOR_PORT: u16 = 5004;
const API_VERSION: &str = "1.0.0";

pub struct UnityEnvironment {
    port: u16,
    first_message_sent: bool,
    behavior_specs: HashMap<String, BehaviorSpec>,
    behavior_actions: HashMap<String, Tensor>,
    communicator: Box<dyn Communicator>,
}

impl<'a> Default for UnityEnvironment {
    fn default() -> UnityEnvironment {
        UnityEnvironment {
            port: DEFAULT_EDITOR_PORT,
            communicator: Box::new(GrpcCommunicator::create()),
            first_message_sent: false,
            behavior_specs: HashMap::new(),
            behavior_actions: HashMap::new()
        }
    }
}

impl Drop for UnityEnvironment {
    fn drop(&mut self) {
        self.communicator.close();
    }
}

impl UnityEnvironment {
    pub fn init(&self) {
        let seed = 1;
        let rl_init_parameters_in = UnityRLInitializationInputProto {
            seed: seed,
            communication_version: String::from(API_VERSION),
            package_version: String::from("0.19.0.dev0"),
            capabilities: SingularPtrField::some(UnityRLCapabilitiesProto {
                baseRLCapabilities: true,
                ..UnityRLCapabilitiesProto::default()
            }),
            ..UnityRLInitializationInputProto::default()
        };
        let aca_output = self.send_academy_parameters(rl_init_parameters_in);
    }

    fn send_academy_parameters(&self, input: UnityRLInitializationInputProto) -> UnityOutputProto {
        let inputs = UnityInputProto {
            rl_initialization_input: SingularPtrField::some(input),
            ..UnityInputProto::default()
        };
        let res = self.communicator.initialize(inputs);
        match res {
            Ok(output) => output,
            Err(e) => panic!("Could not initialize! {}", e)
        }
    }

    fn generate_reset_input(&self) -> UnityInputProto {
        let mut rl_in = UnityRLInputProto::default();
        rl_in.set_command(CommandProto::RESET);
        Self::wrap_unity_input(rl_in)
    }

    fn wrap_unity_input(rl_in: UnityRLInputProto) -> UnityInputProto {
        let mut input_proto = UnityInputProto::default();
        input_proto.set_rl_input(rl_in);
        input_proto
    }

    fn update_behavior_specs(&self, output: UnityOutputProto) {
        let init_output = output.rl_initialization_output.unwrap();
        let brain_params = init_output.brain_parameters;
        let rl_output = output.rl_output.unwrap();
        for brain_param in brain_params.iter() {
            let maybe_agent_infos = &rl_output.agentInfos.get(&brain_param.brain_name);
            if let Some(agent_infos) = maybe_agent_infos {
                let agent = &agent_infos.value[0];
                let new_spec = BehaviorSpec::from_proto(brain_param, agent);
            }
        }
    }
}

impl Environment for UnityEnvironment {
    fn step(&self) -> EnvironmentResult<()> {
        if !self.first_message_sent {
            return self.reset();
        }
        for (behavior_name, behavior_spec) in self.behavior_specs.iter() {
            if !self.behavior_actions.contains_key(behavior_name) {
                let n_agents = 0;
            }
        }
        Ok(())
    }

    fn reset(&self) -> EnvironmentResult<()> {
        let outputs = self.communicator.exchange(self.generate_reset_input());
        println!("Sent reset!");
        match outputs {
            Err(e) => Err(EnvironmentError::CommunicatorExited),
            Ok(o) => {
                self.update_behavior_specs(o);
                Ok(())
            }
        }
    }

    fn set_actions(&mut self, behavior_name: String, action: Tensor) {
        self.behavior_actions.insert(behavior_name, action);
    }

    fn get_steps(&self, behavior_name: String) -> (Vec<DecisionStep>, Vec<TerminalStep>) {
        (vec![], vec![])
    }
}