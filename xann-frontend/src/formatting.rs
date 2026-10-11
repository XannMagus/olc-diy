use pretty::{BoxAllocator, DocBuilder};
use std::fmt::{Display, Formatter};

pub trait ToDoc {
    fn to_doc<'a>(&'a self, allocator: &'a BoxAllocator) -> DocBuilder<'a, BoxAllocator>;
}

pub struct Displayable<T>(pub T);

impl<T: ToDoc> Display for Displayable<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let allocator = BoxAllocator;
        let str = self.0.to_doc(&allocator).pretty(80).to_string();
        write!(f, "{str}")
    }
}
