use std::os::raw::c_char;

pub type NodeIndex = usize;
pub type TransitionIndex = usize;

trait FSM {
    fn new(nameFSM: &str) -> FiniteStateMachine;
    fn addNode(&mut self, name: i32) -> NodeIndex;
    fn removeNode(&mut self, name: i32) -> NodeIndex;
    fn addTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex;
    fn removeTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex;
    fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool;
    fn displayTransition(&self);
    fn process_fsm(&mut self, nodeIndex: NodeIndex, word: &str) -> Vec<(usize, usize)>;
    fn print_vec(mut vec : Vec<Vec<&str>>);
}

struct NamedElement{
    name: String
}

struct SimpleFiniteStateMachine {
    nameFSM: String,
    transitions: Vec<Transition>,
    nodes: Vec<Node>
}

struct Node {
    pub name: i32,
    outputTransition: Vec<TransitionIndex>,
    inputTransition: Vec<TransitionIndex>
}

struct Transition {
    pub letter: char,
    outputNodes: NodeIndex,
    inputNodes: NodeIndex,
}

impl FSM for SimpleFiniteStateMachineFiniteStateMachine {
    fn new(nameFSM: &str) -> FiniteStateMachine {
        return FiniteStateMachine {
            nameFSM: nameFSM.into(),
            transitions: Vec::new(),
            nodes: Vec::new()
        };
    }

    fn addNode(&mut self, name: i32) -> NodeIndex {
        let index: usize = self.nodes.len();
        self.nodes.push(Node {
            name,
            outputTransition: Vec::new(),
            inputTransition: Vec::new(),
        });
        return index;
    }

    fn addTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex {
        let index: usize = self.transitions.len();
        let new_trans = Transition {
            letter,
            outputNodes,
            inputNodes,
        };
        self.transitions.push(new_trans);

        self.nodes[inputNodes].outputTransition.push(index);
        self.nodes[outputNodes].inputTransition.push(index);
        return index;
    }


    fn existTransition(&mut self, transitionTest: TransitionIndex, c_char: char) -> bool {
        return self.transitions[transitionTest].letter == c_char;
    }

    fn displayTransition(&self) {
        println!("\n");
        for transitionNumber in &self.transitions {
            println!("{} {} {} {} {}", self.nodes[transitionNumber.inputNodes].name, "-", transitionNumber.letter, "->", self.nodes[transitionNumber.outputNodes].name);
        }
    }

    fn displayFSM(&self) {
        println!("{}", "\nDisplay Finite state machine : \n");
        print!("State : ");

        for nodeNumber in &self.nodes {
            print!("{} {}", nodeNumber.name, " ");
        }

        self.displayTransition()
    }


    fn process_fsm(&mut self, nodeIndex: NodeIndex, word: &str) -> Vec<(usize, usize)> {
        let mut vec_edge = Vec::new();

        let octets = word.as_bytes();

        let mut iterator_word = 0;
        let mut iterator_trans = 0;
        let mut state = 0;

        for (iterator_word, &char) in octets.iter().enumerate() {
            let char_trans = self.transitions[self.nodes[nodeIndex].outputTransition[iterator_trans]].letter;

            while iterator_trans < self.nodes[nodeIndex].outputTransition.len() && char_trans != char as char {
                iterator_trans += 1;
            }
            if self.transitions[iterator_trans].outputNodes < self.nodes.len() {
                vec_edge.push((state,self.transitions[iterator_trans].outputNodes));
                state = iterator_trans;
                iterator_trans = 0;
            }
        }

        if vec_edge.len() < word.len(){
            println!("Pas de chemin complet trouver pour ce mot.")
        } else {
            println!("Chemin trouvé pour ce mot.")
        }
        return vec_edge;

    }

    fn print_vec(mut vec : Vec<Vec<&str>>) {
        let state_nb = vec.len();

        for i in 0..state_nb {
            for j in 0..state_nb {
                print!("{}", vec[i][j]);
            }
            println!();
        }
        println!();
    }

    fn removeNode(&mut self, name: i32) -> NodeIndex {
        todo!()
    }

    fn removeTransition(&mut self, letter: char, inputNodes: NodeIndex, outputNodes: NodeIndex) -> TransitionIndex {
        todo!()
    }
}
/*
       for edge_index in &self.transitions[trans_index].input_edges {
            let place_index = &self.edges[*edge_index].place;
            self.places[*place_index].tokens -= &self.edges[*edge_index].weight;
        }
        for edge_index in &self.transitions[trans_index].output_edges {
            let place_index = &self.edges[*edge_index].place;
            self.places[*place_index].tokens += &self.edges[*edge_index].weight;
        }
    }*/



fn main() {

    let mut test = FiniteStateMachine::new("Test");
    let name = 0;
    let name2 = 1;
    let name3 = 2;

    test.addNode(name);
    test.addNode(name2);
    test.addNode(name3);

    test.addTransition('a',0,1);
    test.addTransition('b',0,2);
    test.addTransition('c',1,2);
    test.addTransition('d',2,2);

    test.displayFSM();

    let word1 = "acd";
    let mut result = test.process_fsm(0,word1);
    let lenght = result.len();
    print!("Taille du chemin pour ce mot.");
    print!("{}",lenght);
    dbg!(result);
}