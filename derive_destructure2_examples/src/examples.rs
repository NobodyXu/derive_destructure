use derive_destructure2::*;

#[derive(destructure, remove_trait_impls)]
pub struct ImplementsDrop {
    some_str: String,
    some_int: i32,
}

impl Drop for ImplementsDrop {
    fn drop(&mut self) {
        panic!("We don't want to drop this");
    }
}
