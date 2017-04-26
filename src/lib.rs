//! Have docs for items exist in separate files and inserted at compile time for
//! rustdoc

#![feature(plugin_registrar, rustc_private, type_ascription)]
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::registry::Registry;

use syntax::ast::Name;
use syntax::ext::base::{ AttrProcMacro, ExtCtxt, SyntaxExtension };
use syntax::codemap::Span;
use syntax::tokenstream::TokenStream;
use syntax::tokenstream::TokenTree;
use syntax::parse::token::{ Lit, Token };

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(  Name::intern("rdoc"),
                                    SyntaxExtension::AttrProcMacro(
                                        Box::new(Expansion)
                                    )
                                 );
}

struct Expansion;

impl AttrProcMacro for Expansion {
    fn expand<'cx>(&self,
                   _: &'cx mut ExtCtxt,
                   _: Span,
                   annotation: TokenStream,
                   annotated: TokenStream) -> TokenStream

    {
        if !annotation.is_empty() {
            let mut trees = annotation.into_trees();
            if let Some(TokenTree::Token(_, val)) = trees.next() {
                if val == Token::Eq {
                    if let Some(TokenTree::Token(_, val)) = trees.next() {
                        if let Token::Literal(Lit::Str_(path) ,_) = val {
                            let out = TokenStream::concat(
                                vec![
                                      TokenStream::from(
                                        Token::DocComment(
                                            Name::intern(
                                                &docs_gen(&path.as_str())
                                            )
                                        )
                                      )
                                    , annotated
                                    ]
                                );
                            out
                        } else {
                            annotated
                        }
                    } else {
                        annotated
                    }
                } else {
                    annotated
                }

            } else {
                annotated
            }
        } else {
            annotated
        }
    }
}

fn docs_gen(path: &str) -> String {
    let mut buf_reader = BufReader::new( File::open(path)
                                              .expect(
                                                  &format!("Failed to open {} \
                                                           while expanding out \
                                                           documentation", path)
                                              )
                                       );
    let mut comments = String::new() + "///";
    let _ = buf_reader.read_to_string(&mut comments);
    comments
}
