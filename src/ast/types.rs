use super::Ident;

#[derive(Debug, PartialEq, Clone)]
pub enum Type<'ast> {
    /// The regular type
    Ident(Ident<'ast>),
    /// Pointer to a type
    Pointer {
        data_type: &'ast Type<'ast>,
        is_const: bool,
        is_restricted: bool,
    },
    /// Array of a type
    Array {
        data_type: &'ast Type<'ast>,
        size: Option<usize>,
    },
    /// Struct pointer
    Struct(Ident<'ast>),
    /// Union pointer
    Union(Ident<'ast>),
    /// Enum Pointer
    Enum(Ident<'ast>),
}
