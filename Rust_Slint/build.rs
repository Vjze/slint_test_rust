fn main() {
    // let config =
    // slint_build::CompilerConfiguration::new()
    // .with_style("material".into());
    // slint_build::compile_with_config("src/main.slint",config).unwrap();
    slint_build::compile("ui/main.slint").unwrap();
}