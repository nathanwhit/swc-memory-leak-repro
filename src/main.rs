use std::{path::PathBuf, sync::Arc};

use swc_common::SourceFile;
use swc_ecma_transforms_base::hygiene::hygiene;
use swc_ecma_visit::FoldWith;

fn main() {
    let mut errs = Vec::new();

    let src = SourceFile::new(
        PathBuf::from("foo.js").into(),
        false,
        PathBuf::from("foo.js").into(),
        "console.log(\"Hello World!\");".into(),
        swc_common::BytePos(1),
    );
    let program = swc_ecma_parser::parse_file_as_program(
        &src,
        swc_ecma_parser::Syntax::Es(Default::default()),
        swc_ecma_ast::EsVersion::EsNext,
        None,
        &mut errs,
    )
    .unwrap();
    let globals = Arc::new(swc_common::Globals::new());
    let _prog = swc_common::GLOBALS.set(&globals, || program.fold_with(&mut hygiene()));
}
