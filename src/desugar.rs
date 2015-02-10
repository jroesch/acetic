use syntax::ast;
use syntax::visit::{self, Visitor};

use core;

#[derive(Debug)]
pub struct Desugar {
    pub items: Vec<core::Item>
}

impl<'v> Desugar {
    fn desugar_item(&mut self, i: &'v ast::Item) {
        match i.node {
            ast::Item_::ItemFn(..) => self.items.push(core::Item::Fn),
            ast::Item_::ItemTrait(..) => self.items.push(core::Item::Trait),
            ast::Item_::ItemStruct(..) => self.items.push(core::Item::Struct),
            _ => panic!("Item type not supported!")
        }
    }
}

impl<'v> Visitor<'v> for Desugar {
    fn visit_item(&mut self, i: &'v ast::Item) {
        self.desugar_item(i);
        visit::walk_item(self, i);
    }
}
