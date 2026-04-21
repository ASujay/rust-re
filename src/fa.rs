use crate::parser::RegExNode;
use std::{collections::HashMap};

type StateId = usize;

pub struct State {
	pub id: StateId,
	transitions: HashMap::<char, StateId>,
	epsilon: Vec<StateId>,
}

pub struct NfaFragment {
	start: StateId,
	accept: StateId,
}

pub struct Nfa {
	states: Vec<State>,
}

impl Nfa {
	pub fn new() -> Nfa {
		let nfa = Nfa {
			states: Vec::new(),
		};
		return nfa;
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
			RegExNode::Question((node)) => {
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
		self.states[from].transitions.insert(on, to);
	}

	pub fn add_epsilon(&mut self, from: StateId, to: StateId) {
		self.states[from].epsilon.push(to);
	}
}