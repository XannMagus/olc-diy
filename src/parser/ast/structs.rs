use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use crate::parser::ast::Type;
use crate::parser::utilities::ToDoc;

#[derive(Debug)]
pub struct StructFieldInfo {
    name: String,
    field_type: Type,
    is_static: bool,
}

impl StructFieldInfo {
    pub fn new(name: String, field_type: Type, is_static: bool) -> Self {
        Self {
            name,
            field_type,
            is_static,
        }
    }

    pub(super) fn new_static(name: String, field_type: Type) -> Self {
        Self::new(name, field_type, true)
    }

    pub(super) fn new_instance(name: String, field_type: Type) -> Self {
        Self::new(name, field_type, false)
    }
}

impl ToDoc for StructFieldInfo {
    fn to_doc<'a>(&'a self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator> {
        allocator.concat([
            allocator.text("Field").append(allocator.line()),

            allocator.text("Name: ").nest(2),
            allocator.text(self.name.clone()).nest(2),
            allocator.line(),

            allocator.text("Type: ").nest(2),
            self.field_type.to_doc(allocator).nest(2),
            allocator.line(),

            allocator.text("Static: ").nest(2),
            allocator.as_string(self.is_static).nest(2),
            allocator.line(),
        ])
    }
}

impl ToDoc for Vec<StructFieldInfo> {
    fn to_doc<'a>(&'a self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator> {
        let mut docs = vec![];
        for field_info in self.into_iter() {
            docs.push(field_info.to_doc(allocator).nest(2));
        }
        allocator.concat(docs)
    }
}