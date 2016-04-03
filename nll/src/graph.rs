use lalrpop_intern::intern;
use graph_algorithms as ga;
use nll_repr::repr;
use std::collections::HashMap;
use std::iter;
use std::slice;

pub struct FuncGraph<'arena> {
    func: repr::Func<'arena>,
    start_block: BasicBlockIndex,
    blocks: Vec<repr::BasicBlock>,
    block_indices: HashMap<repr::BasicBlock, BasicBlockIndex>,
    successors: Vec<Vec<BasicBlockIndex>>,
    predecessors: Vec<Vec<BasicBlockIndex>>,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BasicBlockIndex {
    index: usize
}

impl<'arena> FuncGraph<'arena> {
    pub fn new(func: repr::Func<'arena>) -> Self {
        let blocks: Vec<_> =
            func.data.keys().cloned().collect();
        let block_indices: HashMap<_, _> =
            func.data.keys()
                     .cloned()
                     .enumerate()
                     .map(|(index, block)| (block, BasicBlockIndex {
                         index: index
                     }))
                     .collect();
        let mut predecessors: Vec<_> =
            (0..blocks.len()).map(|_| Vec::new())
                             .collect();
        let mut successors: Vec<_> =
            (0..blocks.len()).map(|_| Vec::new())
                             .collect();

        for (block, &index) in &block_indices {
            let data = &func.data[block];
            for successor in &data.successors {
                let successor_index = block_indices[successor];
                successors[index.index].push(successor_index);
                predecessors[successor_index.index].push(index);
            }
        }

        let start_name = intern("START");
        let start_block = block_indices[&repr::BasicBlock(start_name)];

        FuncGraph {
            func: func,
            blocks: blocks,
            start_block: start_block,
            block_indices: block_indices,
            predecessors: predecessors,
            successors: successors,
        }
    }

    pub fn func(&self) -> &repr::Func<'arena> {
        &self.func
    }

    pub fn block_data(&self, index: BasicBlockIndex) -> &repr::BasicBlockData {
        let block = self.blocks[index.index];
        &self.func.data[&block]
    }
}

impl<'arena> ga::Graph for FuncGraph<'arena> {
    type Node = BasicBlockIndex;

    fn num_nodes(&self) -> usize {
        self.func.data.len()
    }

    fn start_node(&self) -> BasicBlockIndex {
        self.start_block
    }

    fn predecessors<'graph>(&'graph self, node: BasicBlockIndex)
                            -> <Self as ga::GraphPredecessors<'graph>>::Iter {
        self.predecessors[node.index].iter().cloned()
    }

    fn successors<'graph>(&'graph self, node: BasicBlockIndex)
                          -> <Self as ga::GraphSuccessors<'graph>>::Iter {
        self.successors[node.index].iter().cloned()
    }
}

impl<'arena, 'graph> ga::GraphPredecessors<'graph> for FuncGraph<'arena> {
    type Item = BasicBlockIndex;
    type Iter = iter::Cloned<slice::Iter<'graph, BasicBlockIndex>>;
}

impl<'arena, 'graph> ga::GraphSuccessors<'graph> for FuncGraph<'arena> {
    type Item = BasicBlockIndex;
    type Iter = iter::Cloned<slice::Iter<'graph, BasicBlockIndex>>;
}

impl ga::NodeIndex for BasicBlockIndex {
}

impl From<usize> for BasicBlockIndex {
    fn from(v: usize) -> BasicBlockIndex {
        BasicBlockIndex {
            index: v
        }
    }
}

impl Into<usize> for BasicBlockIndex {
    fn into(self) -> usize {
        self.index
    }
}