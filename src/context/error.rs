use crate::context::ContextSchemaMismatch;

pub type ContextResult<T> = Result<T, ContextError>;

pub enum ContextError {
    SchemaMismatch { fields: Vec<ContextSchemaMismatch> },
}
