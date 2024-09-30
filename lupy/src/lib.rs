use full_moon;
use rustpython_parser;

pub fn into_python(luau: &String) {
  let ast = full_moon::parse(luau).unwrap();
  let mut python = rustpython_parser::ast::Ast;

  for (i, node) in ast.nodes().stmts().enumerate() {
    println!("NODE {} {:?}", i, node)
  }
}
