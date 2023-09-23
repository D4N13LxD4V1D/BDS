fn main() {
    peginator_codegen::Compile::file("bds.ebnf")
        .destination("src/grammar.rs")
        .format()
        .run_exit_on_error();

    println!("cargo:rerun-if-changed=bds.ebnf");
}
