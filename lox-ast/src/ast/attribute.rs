use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Attribute {
    id: NodeId,
    span: Span,
}

impl Attribute {
    pub fn new(id: NodeId, span: Span) -> Self {
        Self { id, span }
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug, Clone)]
pub struct NodeId(u32);

impl NodeId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }
}
