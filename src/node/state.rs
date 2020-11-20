use dasp_graph::{Buffer, Input, Node};
use super::super::{Pairs, Rule, NodeData, BoxedNodeSend, EngineError};

pub struct State {
    // sig: Box<dyn Signal<Frame=f64> + Send>
    info: Vec::<Vec<f32>>,
    state: usize,
    step: usize
}

impl State {
    pub fn new(paras: &mut Pairs<Rule>) -> Result<
    (NodeData<BoxedNodeSend>, Vec<String>), EngineError> {
        
        let coma_sep: Vec<&str> = paras.as_str().split(",").collect();
       
        let info = coma_sep.into_iter().map(|s|
            s.split(" ")
            .filter(|s|s!=&"")
            .map( |s| s.parse::<f32>().unwrap()) // TODO: error handling
            .collect::<Vec<f32>>()
        ).collect::<Vec<Vec<f32>>>();
        // println!("{:?}", states);

        Ok((NodeData::new1(BoxedNodeSend::new( Self {
            info,
            state: 0,
            step: 0
        })), vec![]))
    }
}

impl Node for State {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for i in 0..64 {
            if self.state >= self.info.len() - 1 {
                output[0][i] = self.info[self.info.len()-1][1];
            } else {
                let inc = self.info[self.state + 1][1] - self.info[self.state][1];
                let dur = self.info[self.state + 1][0] - self.info[self.state][0];
                let state_time =  self.step as f32 / 44100.0 - self.info[self.state][0];
                output[0][i] = self.info[self.state][1] + state_time / dur * inc;
                if self.step as f32 / 44100.0 > self.info[self.state + 1][0] {
                    self.state += 1;
                }
            }
            self.step += 1;
        }
        // output[0] = inputs[0].buffers()[0].clone();
    }
}