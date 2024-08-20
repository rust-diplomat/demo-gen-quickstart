#[diplomat::bridge]
mod ffi {
    use std::fmt::Write;

	#[diplomat::opaque]
	#[diplomat::rust_link(basic_adder, Mod)]
	pub struct AddResult;

	impl AddResult {
		pub fn get_add_str(left : u32, right : u32, write: &mut DiplomatWrite) {
			write.write_str(&format!("{}", basic_adder::add(left, right))).unwrap();
			write.flush();
		}
	}
}