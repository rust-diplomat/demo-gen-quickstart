use std::path::Path;

use diplomat_tool::DocsUrlGenerator;

pub fn main() {
	diplomat_tool::gen(Path::new("adder_bindings/src/lib.rs"), "demo_gen", Path::new("adder_bindings/demo/demo_gen"), &DocsUrlGenerator::default(), None, false).unwrap();
}