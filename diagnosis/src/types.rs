use std::any::type_name;

// https://users.rust-lang.org/t/how-check-type-of-variable/33845/2
pub fn type_of<T>(_: T) -> &'static str {
	type_name::<T>()
}

#[cfg(test)]
mod tests {
	use super::type_of;

	#[test]
	fn fn_type_of() {
		assert_eq!(type_of(0i32),				"i32");
		
		assert_eq!(type_of(Some(0i32)),				"core::option::Option<i32>");
		assert_eq!(type_of(Some(0i32).as_ref()),		"core::option::Option<&i32>");
		assert_eq!(type_of(Some(0i32).as_mut()),		"core::option::Option<&mut i32>");

		assert_eq!(type_of(Some(Box::new(0i32))),		"core::option::Option<alloc::boxed::Box<i32>>");
		assert_eq!(type_of(Some(Box::new(0i32)).as_deref()),	"core::option::Option<&i32>");
		assert_eq!(type_of(Some(Box::new(0i32)).as_deref_mut()),"core::option::Option<&mut i32>");
	}
}
