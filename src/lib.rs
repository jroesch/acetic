#![feature(rustc_private)]
#![feature(path)]
extern crate syntax;

use syntax::ast;
use syntax::parse;

mod desugar;
mod core;

pub fn parse_crate(file_path: &str) -> ast::Crate {
    let ref sess = parse::new_parse_sess();
    parse::parse_crate_from_file(&Path::new(file_path), Vec::new(), sess)
}

pub fn compile_crate(krate: ast::Crate) {
    let mut desugarer = desugar::Desugar { items: Vec::new() };
    syntax::visit::walk_crate(&mut desugarer, &krate);
    println!("result: {:?}", desugarer);
}
