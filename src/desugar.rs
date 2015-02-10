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
            // ast::Item_::ItemFn(..) => self.items.push(core::Item::Fn),
            // ast::Item_::ItemTrait(..) => self.items.push(core::Item::Trait),
            ast::Item_::ItemStruct(ref struct_def, ref ast_generics) => {
                // Figure out what kind of struct we have.
                let core_struct = match struct_def.ctor_id {
                    // Tuple like struct
                    Some(_)  => { panic!("{:?}", struct_def) }
                    // Record like struct
                    None => {
                        let name = core::Name::from_ident(i.ident);
                        let generics = core::Generics::empty();
                        let predicates = Vec::new();
                        let fields = Vec::new();

                        core::Item::record_struct(
                            name,
                            generics,
                            predicates,
                            fields)
                    }
                };

                self.items.push(core_struct);
            }
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
