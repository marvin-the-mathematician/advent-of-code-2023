advent_of_code::solution!(20);

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::opt,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Finish, IResult,
};
use std::collections::{HashMap, VecDeque};
use std::iter::repeat_with;
use std::str::FromStr;

type Count = usize;
type Name = String;

fn parse_name(input: &str) -> IResult<&str, Name> {
    let (i, name) = alpha1(input)?;
    Ok((i, name.to_string()))
}

type Names = Vec<Name>;
type NamesForName = HashMap<Name, Names>;

fn parse_names(input: &str) -> IResult<&str, Names> {
    let (i, names) = separated_list1(tag(", "), parse_name)(input)?;
    Ok((i, names))
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum State {
    #[default]
    Off,
    On,
}

fn flipped_state(state: State) -> State {
    match state {
        State::Off => State::On,
        State::On => State::Off,
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Pulse {
    #[default]
    Low,
    High,
}

type PulseForName = HashMap<Name, Pulse>;

#[derive(Clone, Debug, Default, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Message {
    sender_name: Name,
    receiver_name: Name,
    pulse: Pulse,
}

type Messages = Vec<Message>;
type MessageQueue = VecDeque<Message>;

trait Handler {
    fn handle(
        &mut self,
        message: &Message,
        name: &Name,
        receiver_names: &Names,
    ) -> Option<Messages>;
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct FlipFlop {
    state: State,
}

impl Handler for FlipFlop {
    fn handle(
        &mut self,
        message: &Message,
        name: &Name,
        receiver_names: &Names,
    ) -> Option<Messages> {
        let Message {
            sender_name: _,
            receiver_name: _,
            pulse: input,
        } = message;
        let maybe_messages = match input {
            Pulse::Low => {
                let output = match self.state {
                    State::Off => Pulse::High,
                    State::On => Pulse::Low,
                };
                self.state = flipped_state(self.state);
                let messages = receiver_names
                    .iter()
                    .map(|receiver_name| Message {
                        sender_name: name.clone(),
                        receiver_name: receiver_name.clone(),
                        pulse: output,
                    })
                    .collect();

                Some(messages)
            }
            Pulse::High => None,
        };

        maybe_messages
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Conjunction {
    last_pulse_for_name: PulseForName,
}

impl Handler for Conjunction {
    fn handle(
        &mut self,
        message: &Message,
        name: &Name,
        receiver_names: &Names,
    ) -> Option<Messages> {
        let Message {
            sender_name: from,
            receiver_name: _,
            pulse: input,
        } = message;
        self.last_pulse_for_name.insert(from.clone(), *input);
        // TODO This is not right!
        // For names that one has not yet seen one should assume Pulse::Low, not Pulse::High :-(
        let all_high = self
            .last_pulse_for_name
            .values()
            .all(|&pulse| pulse == Pulse::High);
        let output = if all_high { Pulse::Low } else { Pulse::High };
        let messages = receiver_names
            .iter()
            .map(|receiver_name| Message {
                sender_name: name.clone(),
                receiver_name: receiver_name.clone(),
                pulse: output,
            })
            .collect();

        Some(messages)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Broadcast {}

impl Handler for Broadcast {
    fn handle(
        &mut self,
        message: &Message,
        name: &Name,
        receiver_names: &Names,
    ) -> Option<Messages> {
        let Message {
            sender_name: _,
            receiver_name: _,
            pulse: input,
        } = message;
        let messages = receiver_names
            .iter()
            .map(|receiver_name| Message {
                sender_name: name.clone(),
                receiver_name: receiver_name.clone(),
                pulse: *input,
            })
            .collect();

        Some(messages)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Relay {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

fn parse_relay(input: &str) -> IResult<&str, Relay> {
    let (i, maybe_prefix) = opt(alt((char('%'), char('&'))))(input)?;
    let category = match maybe_prefix {
        Some('%') => Relay::FlipFlop(FlipFlop { state: State::Off }),
        Some('&') => Relay::Conjunction(Conjunction {
            last_pulse_for_name: PulseForName::new(),
        }),
        None => Relay::Broadcast(Broadcast {}),
        _ => panic!(),
    };
    Ok((i, category))
}

impl Handler for Relay {
    fn handle(
        &mut self,
        message: &Message,
        name: &Name,
        receiver_names: &Names,
    ) -> Option<Messages> {
        match self {
            Relay::FlipFlop(handler) => handler.handle(message, name, receiver_names),
            Relay::Conjunction(handler) => handler.handle(message, name, receiver_names),
            Relay::Broadcast(handler) => handler.handle(message, name, receiver_names),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Module {
    name: Name,
    receiver_names: Names,
    relay: Relay,
}

impl Module {
    fn initialized_clone(&self, sender_names_for_name: &NamesForName) -> Module {
        let mut module = self.clone();
        if let Relay::Conjunction(ref mut conjunction) = module.relay {
            let sender_names = &sender_names_for_name.get(&module.name).unwrap();
            conjunction.last_pulse_for_name = sender_names
                .iter()
                .map(|name| (name.clone(), Pulse::Low))
                .collect();
        }

        module.clone()
    }
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    let (i, ((relay, name), receiver_names)) =
        separated_pair(tuple((parse_relay, parse_name)), tag(" -> "), parse_names)(input)?;
    Ok((
        i,
        Module {
            name,
            receiver_names,
            relay,
        },
    ))
}

type Modules = Vec<Module>;

fn parse_modules(input: &str) -> IResult<&str, Modules> {
    let (i, modules) = separated_list1(newline, parse_module)(input)?;
    Ok((i, modules))
}

type ModuleForName = HashMap<Name, Module>;

#[derive(Debug)]
struct Configuration {
    module_for_name: ModuleForName,
}

fn parse_configuration(input: &str) -> IResult<&str, Configuration> {
    let (i, modules) = parse_modules(input)?;

    // Since modules with conjunction relays need to know which modules may send them a message, one
    // must collect these sender names for each receiver name. Since we don't currently have a way
    // to test whether a given module name is associated with a conjunction relay we just collect
    // them all...
    let sender_names_for_name = (|| {
        let mut result = NamesForName::new();
        modules.iter().for_each(|module| {
            module.receiver_names.iter().for_each(|receiver_name| {
                if !result.contains_key(receiver_name) {
                    result.insert(receiver_name.clone(), Names::new());
                }
                let sender_names = result.get_mut(receiver_name).unwrap();
                sender_names.push(module.name.clone());
            })
        });

        result.clone()
    })();

    let module_for_name = modules
        .iter()
        .map(|module| {
            (
                module.name.clone(),
                module.initialized_clone(&sender_names_for_name),
            )
        })
        .collect();

    Ok((i, Configuration { module_for_name }))
}

impl FromStr for Configuration {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_configuration(s).finish() {
            Ok((_, configuration)) => Ok(configuration),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Configuration {
    fn push_button(&mut self) -> (Count, Count) {
        let mut low_pulse_count: Count = 0;
        let mut high_pulse_count: Count = 0;
        let message = Message {
            sender_name: Name::from("button"),
            receiver_name: Name::from("broadcaster"),
            pulse: Pulse::Low,
        };

        let mut message_queue = MessageQueue::new();
        message_queue.push_back(message);
        while let Some(message) = message_queue.pop_front() {
            println!("{:?}", message);
            let Message {
                sender_name: _,
                receiver_name,
                pulse,
            } = &message;
            match pulse {
                Pulse::Low => low_pulse_count += 1,
                Pulse::High => high_pulse_count += 1,
            }
            if let Some(module) = self.module_for_name.get_mut(receiver_name) {
                if let Some(messages) =
                    module
                        .relay
                        .handle(&message, &module.name, &module.receiver_names)
                {
                    messages
                        .into_iter()
                        .for_each(|message| message_queue.push_back(message));
                }
            }
        }
        println!();

        (low_pulse_count, high_pulse_count)
    }
}

pub fn part_one(input: &str) -> Option<Count> {
    let mut configuration = Configuration::from_str(input).ok()?;
    // println!("{:?}\n", configuration);

    let (low_pulse_count, high_pulse_count) = repeat_with(|| configuration.push_button())
        .take(1000)
        .reduce(|(a, b), (x, y)| (a + x, b + y))?;
    println!(
        "low_pulse_count: {:?}, high_pulse_count: {:?}",
        low_pulse_count, high_pulse_count
    );

    Some(low_pulse_count * high_pulse_count)
}

pub fn part_two(input: &str) -> Option<Count> {
    let mut configuration = Configuration::from_str(input).ok()?;
    // println!("{:?}\n", configuration);

    let (low_pulse_count, high_pulse_count) = repeat_with(|| configuration.push_button())
        .take(1000)
        .reduce(|(a, b), (x, y)| (a + x, b + y))?;
    println!(
        "low_pulse_count: {:?}, high_pulse_count: {:?}",
        low_pulse_count, high_pulse_count
    );

    Some(low_pulse_count * high_pulse_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_trial_one() {
        let result = part_one(&advent_of_code::template::read_file_for_trial(
            "examples", DAY, "1",
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
