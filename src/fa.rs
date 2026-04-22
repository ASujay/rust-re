use crate::parser::RegExNode;
use std::collections::{HashMap, HashSet};

type StateId = usize;

pub struct State {
	pub id: StateId,
	transitions: HashMap::<char, Vec<StateId>>,
	epsilon: Vec<StateId>,
}

pub struct NfaFragment {
	start: StateId,
	accept: StateId,
}

pub struct Nfa {
	states: Vec<State>,
	fragment: NfaFragment,
	line: usize,
}

impl Nfa {
	pub fn new(ast: &RegExNode) -> Nfa {
		let mut nfa = Nfa {
			states: Vec::new(),
			fragment: NfaFragment { start: 0, accept: 0 },
			line: 1,
		};
		nfa.fragment = nfa.build(ast);
		return nfa;
	}

	pub fn find_match(&self, start: usize, content: &str) -> Option<(usize, usize)> {
		let mut current_states = HashSet::<StateId>::new();
		current_states.insert(self.fragment.start);
		current_states = self.epsilon_closure(&current_states);
		let mut last_idx = 0;
		let mut last_match: Option<(usize, usize)> = None;
		for (idx, c) in content[start..].char_indices() {
			last_idx = start + idx + c.len_utf8();
			if current_states.contains(&self.fragment.accept) {
				last_match = Some((start, start + idx));
			}
			
			let mut next_states: HashSet<StateId> = HashSet::new();
			for state in &current_states {
				if let Some(next_state) = self.states[*state].transitions.get(&c) {
					next_states.extend(next_state);
				}
			}
			current_states = self.epsilon_closure(&next_states);

			if current_states.is_empty() {
				break;
			}
		}

		if current_states.contains(&self.fragment.accept) {
			last_match = Some((start, last_idx));
		}

		last_match
	}

	pub fn search(&mut self, content: &str) -> Vec<(usize, usize, usize)> {
		let mut result: Vec<(usize, usize, usize)> = Vec::new();
		for (idx, ch) in content.char_indices() {
			if ch == '\n' {
				self.line += 1;
			}
			if let Some((start, end)) = self.find_match(idx, content) {
				if start != end {
					result.push((start, end, self.line));
				}
			}
		}
		result
	}

	fn epsilon_closure(&self, states: &HashSet<StateId>) -> HashSet<StateId> {
		let mut result = states.clone();
		let mut stack: Vec<_> = states.iter().cloned().collect();

		while let Some(state) = stack.pop() {
			for &next in &self.states[state].epsilon {
				if result.insert(next) {
					stack.push(next);
				}
			}
		}
		return result;
	}

	pub fn build(&mut self, ast: &RegExNode) -> NfaFragment {
		match ast {
			RegExNode::Character(chr) => {
				let start = self.new_state();
				let accept = self.new_state();
				self.add_transition(start, *chr, accept);
				NfaFragment { start, accept }
			},
			RegExNode::Plus(node) => {
				let new_start = self.new_state();
				let new_accept = self.new_state();
				let child_frag = self.build(node.as_ref());
				self.add_epsilon(new_start, child_frag.start);
				self.add_epsilon(child_frag.accept, new_accept);
				self.add_epsilon(child_frag.accept, child_frag.start);
				NfaFragment { start: new_start, accept: new_accept }
			},
			RegExNode::Question(node) => {
				let new_start = self.new_state();
				let new_accept = self.new_state();
				let child_frag = self.build(node.as_ref());
				self.add_epsilon(new_start, child_frag.start);
				self.add_epsilon(child_frag.accept, new_accept);
				self.add_epsilon(new_start, new_accept);
				NfaFragment { start: new_start, accept: new_accept }
			},
			RegExNode::Star(node) => {
				// zero or more
				let new_start = self.new_state();
				let new_accept = self.new_state();
				let child_frag = self.build(node.as_ref());
				self.add_epsilon(new_start, child_frag.start);
				self.add_epsilon(child_frag.accept, new_accept);
				self.add_epsilon(new_start, new_accept);
				self.add_epsilon(child_frag.accept, new_start);
				NfaFragment { start: new_start, accept: new_accept }
			},
			RegExNode::Concat(l_node, r_node) => {
				let l_frag = self.build(l_node.as_ref());
				let r_frag = self.build(r_node.as_ref());
				self.add_epsilon(l_frag.accept, r_frag.start);
				return NfaFragment { start: l_frag.start, accept: r_frag.accept };
			},
			RegExNode::Alternation(l_node, r_node) => {
				let new_start = self.new_state();
				let new_accept = self.new_state();
				let l_frag = self.build(l_node.as_ref());
				let r_frag = self.build(r_node.as_ref());
				self.add_epsilon(new_start, l_frag.start);
				self.add_epsilon(new_start, r_frag.start);
				self.add_epsilon(l_frag.accept, new_accept);
				self.add_epsilon(r_frag.accept, new_accept);
				NfaFragment { start: new_start, accept: new_accept }
			},
		}
	}

	pub fn new_state(&mut self) -> StateId {
		let id = self.states.len();
		self.states.push(State { 
			id, 
			transitions: HashMap::new(), 
			epsilon: Vec::new() 
		});
		return id;
	}

	pub fn add_transition(&mut self, from: StateId, on: char, to: StateId) {
		self.states[from].transitions.entry(on).or_insert_with(Vec::new).push(to);
	}

	pub fn add_epsilon(&mut self, from: StateId, to: StateId) {
		self.states[from].epsilon.push(to);
	}
}