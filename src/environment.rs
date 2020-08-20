use super::behavior::*;
use super::communicator::*;
use super::communicator_objects as co;
use co::capabilities::UnityRLCapabilitiesProto;
use co::command::*;
use co::unity_input::UnityInputProto;
use co::unity_output::UnityOutputProto;
use co::unity_rl_initialization_input::UnityRLInitializationInputProto;
use co::unity_rl_input::*;
use co::unity_rl_output::*;
use co::agent_action::AgentActionProto;
use co::agent_info::*;
use co::observation::ObservationProto;
use protobuf::{SingularPtrField, RepeatedField};
use std::collections::HashMap;
// use tch::Tensor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvironmentError {
    #[error("Communicator has exited.")]
    CommunicatorExited,
}

pub type EnvironmentResult<T> = Result<T, EnvironmentError>;

pub trait Environment {
    fn step(&mut self) -> EnvironmentResult<()>;
    fn reset(&mut self) -> EnvironmentResult<()>;
    fn set_actions(&mut self, behavior_name: String, action: Vec<Vec<f32>>);
    fn get_steps(&self, behavior_name: String) -> (Vec<DecisionStep>, Vec<TerminalStep>);
}

const DEFAULT_EDITOR_PORT: u16 = 5004;
const API_VERSION: &str = "1.0.0";

pub struct UnityEnvironment {
    port: u16,
    first_message_sent: bool,
    behavior_specs: HashMap<String, BehaviorSpec>,
    behavior_actions: HashMap<String, Vec<Vec<f32>>>,
    behavior_steps: HashMap<String, (Vec<DecisionStep>, Vec<TerminalStep>)>,
    communicator: Box<dyn Communicator>,
}

impl<'a> Default for UnityEnvironment {
    fn default() -> UnityEnvironment {
        UnityEnvironment {
            port: DEFAULT_EDITOR_PORT,
            communicator: Box::new(GrpcCommunicator::create(DEFAULT_EDITOR_PORT)),
            first_message_sent: false,
            behavior_specs: HashMap::new(),
            behavior_actions: HashMap::new(),
            behavior_steps: HashMap::new(),
        }
    }
}

impl Drop for UnityEnvironment {
    fn drop(&mut self) {
        self.communicator.close();
    }
}

fn steps_from_proto(
    agent_info_list: &RepeatedField<AgentInfoProto>,
    behavior_spec: &BehaviorSpec
) -> (Vec<DecisionStep>, Vec<TerminalStep>) {
    let mut decision_agent_infos = agent_info_list.clone();
    decision_agent_infos.retain(|x| !x.done);
    let mut terminal_agent_infos = agent_info_list.clone();
    terminal_agent_infos.retain(|x| x.done);
    // let mut decision_obs_list = vec![];
    // let mut terminal_obs_list = vec![];
    let mut decision_steps = vec![];
    let mut terminal_steps = vec![];
    // for i in 0..behavior_spec.obs_space.len() {
    //     let is_visual = behavior_spec.obs_space[i].len() == 3;
    //     if is_visual {
    //         // TODO: process visual obs
    //         panic!("Visual not supported yet");
    //     }
    //     else {
    //         // TODO: process vector obs
            
    //     }
    // }
    for agent_info in decision_agent_infos.iter() {
        decision_steps.push(DecisionStep {
            obs: vec![],
            agent_id: agent_info.id,
            reward: agent_info.reward,
            action_mask: None
        });
    }
    for agent_info in terminal_agent_infos.iter() {
        terminal_steps.push(TerminalStep {
            obs: vec![],
            agent_id: agent_info.id,
            reward: agent_info.reward,
            interrupted: agent_info.max_step_reached
        });
    }
    (decision_steps, terminal_steps)
}

impl UnityEnvironment {
    pub fn init(&mut self) {
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
        self.update_behavior_specs(aca_output)
    }

    fn send_academy_parameters(&self, input: UnityRLInitializationInputProto) -> UnityOutputProto {
        let inputs = UnityInputProto {
            rl_initialization_input: SingularPtrField::some(input),
            ..UnityInputProto::default()
        };
        let res = self.communicator.initialize(inputs);
        match res {
            Ok(output) => output,
            Err(e) => panic!("Could not initialize! {}", e),
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

    fn update_behavior_specs(&mut self, output: UnityOutputProto) {
        let maybe_init_output = output.rl_initialization_output.into_option();
        let maybe_rl_output = output.rl_output.into_option();
        if let (Some(init_output), Some(rl_output)) = (maybe_init_output, maybe_rl_output) {
            println!("Had both initialization and RL output!");
            let brain_params = init_output.brain_parameters;
            for brain_param in brain_params.iter() {
                println!("Brain name in update behavior specs: {}", &brain_param.brain_name);
                println!("Agent infos keys: {:?}", &rl_output.agentInfos.keys());
                let maybe_agent_infos = &rl_output.agentInfos.get(&brain_param.brain_name);
                if let Some(agent_infos) = maybe_agent_infos {
                    let agent = &agent_infos.value[0];
                    let new_spec = BehaviorSpec::from_proto(brain_param, agent);
                    self.behavior_specs
                        .insert(brain_param.brain_name.clone(), new_spec);
                    println!("Connected new brain:\n{}", brain_param.brain_name);
                }
            }
        }
    }

    fn generate_step_input(&self) -> UnityInputProto {
        let mut actions_by_behavior = HashMap::new();
        for (behavior_name, action_vec) in self.behavior_actions.iter() {
            let n_agents = match self.behavior_steps.get(behavior_name) {
                Some((decision_steps, _)) => decision_steps.len(),
                None => 0
            };
            let mut agent_actions: Vec<AgentActionProto> = vec![];
            for i in 0..n_agents {
                println!("{:?}", action_vec[i].clone());
                let action = AgentActionProto {
                    vector_actions: action_vec[i].clone(),
                    ..AgentActionProto::default()
                };
                agent_actions.push(action);
            }
            let list_agent_action = UnityRLInputProto_ListAgentActionProto {
                value: RepeatedField::from_vec(agent_actions),
                ..UnityRLInputProto_ListAgentActionProto::default()
            };
            actions_by_behavior.insert(behavior_name.clone(), list_agent_action);
        }
        // TODO: add side channel bytes here
        let rl_input = UnityRLInputProto {
            agent_actions: actions_by_behavior,
            command: CommandProto::STEP,
            ..UnityRLInputProto::default()
        };
        UnityInputProto {
            rl_input: SingularPtrField::some(rl_input),
            ..UnityInputProto::default()
        }
    }

    fn update_state(&mut self, output: &UnityRLOutputProto) {
        for (behavior_name, behavior_spec) in self.behavior_specs.iter() {
            match output.agentInfos.get(behavior_name) {
                Some(list_agent_info) => {
                    let agent_info_list = &list_agent_info.value;
                    self.behavior_steps.insert(
                        behavior_name.clone(), steps_from_proto(&agent_info_list, &behavior_spec)
                    );
                },
                None => {
                    self.behavior_steps.insert(
                        behavior_name.clone(), (vec![], vec![])
                    );
                }
            }
        }
    }
}

impl Environment for UnityEnvironment {
    fn step(&mut self) -> EnvironmentResult<()> {
        if !self.first_message_sent {
            return self.reset();
        }
        let mut rng = rand::thread_rng();
        for (behavior_name, behavior_spec) in self.behavior_specs.iter() {
            if !self.behavior_actions.contains_key(behavior_name) {
                let n_agents = match self.behavior_steps.get(behavior_name) {
                    Some((decision_steps, _final_steps)) => decision_steps.len(),
                    None => 0,
                };
                let actions = behavior_spec.action_space.random_actions(n_agents, &mut rng);
                println!("Missing behavior actions, sending: {:?}", actions);
                self.behavior_actions.insert(
                    behavior_name.clone(),
                    actions
                    // behavior_spec.create_empty_action(n_agents),
                );
            }
        }
        let step_input = self.generate_step_input();
        self.communicator.exchange(step_input)
            .map(move |output| {
                self.update_behavior_specs(output.clone());
                match output.rl_output.into_option() {
                    Some(rl_output) => self.update_state(&rl_output),
                    None => panic!("No RL output after step!")
                }
                self.behavior_actions.clear();
                ()
            })
            .map_err(|_| {
                println!("Error stepping.");
                EnvironmentError::CommunicatorExited
            })
    }

    fn reset(&mut self) -> EnvironmentResult<()> {
        let outputs = self.communicator.exchange(self.generate_reset_input());
        println!("Sent reset!");
        match outputs {
            Err(e) => Err(EnvironmentError::CommunicatorExited),
            Ok(o) => {
                self.first_message_sent = true;
                self.update_behavior_specs(o);
                Ok(())
            }
        }
    }

    fn set_actions(&mut self, behavior_name: String, action: Vec<Vec<f32>>) {
        self.behavior_actions.insert(behavior_name, action);
    }

    fn get_steps(&self, behavior_name: String) -> (Vec<DecisionStep>, Vec<TerminalStep>) {
        (vec![], vec![])
    }
}
