use super::types::NodeId;

pub trait Router {
    fn next_hop(&self, dst: NodeId) -> Option<NodeId>;
}

pub struct NoRouter;

impl Router for NoRouter {
    fn next_hop(&self, _dst: NodeId) -> Option<NodeId> {
        None
    }
}

