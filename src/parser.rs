use syntax::parse::parser::Parser;
use syntax::ext::build::AstBuilder;
use syntax::codemap::Span;
use syntax::parse;
use syntax::parse::common::{seq_sep_trailing_allowed};
use syntax::parse::token;
use syntax::parse::token::{Token, intern};
use syntax::parse::token::DelimToken::{Brace};
use syntax::ast;
use syntax::ast::{StructVariantKind, TupleVariantKind, TokenTree, Ident, TtToken, Item, Item_, StructDef, EnumDef, Generics, Variant, Variant_};
use syntax::ptr::P;
use syntax::ast::Item_::*;
use syntax::ext::base::{ExtCtxt, MacResult, MacItems, DummyResult, MacExpr, SyntaxExtension, TTMacroExpander};
use syntax::codemap::{Spanned, spanned};
use rustc::plugin::Registry;


//pub trait ParserExt {

//}

//impl<'a> ParserExt for Parser<'a> {
    //fn parse_dt_item(&mut self) -> Option<dt::Item> {
        //match self.parse_item(vec![]) {
            //Some(item) => {
                //Some(dt::Item::RustItem(item.and_then(|i| i)))
            //}
            //None => {
                //if self.parse_data_kw() {
                    //let name = self.parse_ident();
                    //self.eat(&Token::Colon);
                    //let ty = self.parse_ty();
                    //self.eat(&Token::OpenDelim(Brace));
                    //let constructors = self.parse_data_decl_body();
                    //self.eat(&Token::CloseDelim(Brace));
                    //Some(dt::Item::DataType(name, constructors))
                //} else {
                    //None
                //}
            //}
        //}
    //}

    //fn parse_data_kw(&mut self) -> bool {
        //match self.token {
            //Token::Ident(id, _) if id.as_str() == "data" => {
                //self.bump(); true
            //}
            //_ => false
        //}
    //}

    //fn parse_data_decl_body(&mut self /* , _generics: &ast::Generics */) -> dt::Constructors {
        //let mut constructors = Vec::new();
        //let mut all_nullary = true;
        //// let mut any_disr = None;
        //while self.token != token::CloseDelim(token::Brace) {
            //// let variant_attrs = Vec::new(); //self.parse_outer_attributes();
            //let vlo = self.span.lo;

            //// let vis = self.parse_visibility();

            //// let kind;
            //// let mut args = Vec::new();
            //// let mut disr_expr = None;
            //let ident = self.parse_ident();
            //self.eat(&Token::Colon);
            //let ty = self.parse_dt_ty();

            //let cons = dt::Constructor::TypeConstructor(ident, ty);
            //constructors.push(cons);

            //if !self.eat(&token::Comma) { break; }
        //}
        ////     if self.eat(&token::OpenDelim(token::Brace)) {
        ////         // Parse a struct variant.
        ////         all_nullary = false;
        ////         let start_span = self.span;
        ////         let struct_def = self.parse_struct_def();
        ////         if struct_def.fields.len() == 0 {
        ////             self.span_err(start_span,
        ////                 format!("unit-like struct variant should be written \
        ////                     without braces, as `{},`",
        ////                     token::get_ident(ident))[]);
        ////         }
        ////         kind = StructVariantKind(struct_def);
        ////     } else if self.check(&token::OpenDelim(token::Paren)) {
        ////         all_nullary = false;
        ////         let arg_tys = self.parse_enum_variant_seq(
        ////             &token::OpenDelim(token::Paren),
        ////             &token::CloseDelim(token::Paren),
        ////             seq_sep_trailing_allowed(token::Comma),
        ////             |p| p.parse_ty_sum()
        ////         );
        ////         for ty in arg_tys.into_iter() {
        ////             args.push(ast::VariantArg {
        ////                 ty: ty,
        ////                 id: ast::DUMMY_NODE_ID,
        ////             });
        ////         }
        ////         kind = TupleVariantKind(args);
        ////     } else if self.eat(&token::Eq) {
        ////         disr_expr = Some(self.parse_expr());
        ////         any_disr = disr_expr.as_ref().map(|expr| expr.span);
        ////         kind = TupleVariantKind(args);
        ////     } else {
        ////         kind = TupleVariantKind(Vec::new());
        ////     }
        ////
        ////     let vr = ast::Variant_ {
        ////         name: ident,
        ////         attrs: variant_attrs,
        ////         kind: kind,
        ////         id: ast::DUMMY_NODE_ID,
        ////         disr_expr: disr_expr,
        ////         vis: vis,
        ////     };
        ////
        ////     variants.push(P(spanned(vlo, self.last_span.hi, vr)));
        ////
        ////     if !self.eat(&token::Comma) { break; }
        ////
        //// self.expect(&token::CloseDelim(token::Brace));
        //// match any_disr {
        ////     Some(disr_span) if !all_nullary =>
        ////         self.span_err(disr_span,
        ////             "discriminator values can only be used with a c-like enum"),
        ////     _ => ()
        //// }

        //constructors
    //}

    //fn parse_dt_ty(&mut self) -> dt::Ty {
        //if self.parse_forall() {
            //self.expect(&token::OpenDelim(token::Paren));
            //let mut params = Vec::new();
            //while self.token != token::CloseDelim(token::Paren) {
                //let name = self.parse_ident();
                //self.expect(&token::Colon);
                //let ty = self.parse_ty();
                //if !self.eat(&token::Comma) { break; }
                //params.push(dt::Param { name: name, ty: dt::Ty::RustType(ty.and_then(|t| t.clone())) });
            //}
            //self.expect(&token::CloseDelim(token::Paren));
            //self.expect(&token::RArrow);
            //let ty = self.parse_dt_ty();
            //dt::Ty::Forall(params, box ty)
        //} else {
            //dt::Ty::RustType(self.parse_ty().and_then(|t| t.clone()))
        //}
    //}

    //fn parse_forall(&mut self) -> bool {
        //match self.token {
            //Token::Ident(id, _) if id.as_str() == "forall" => {
                //self.bump(); true
            //}
            //_ => false
        //}
    //}
}
