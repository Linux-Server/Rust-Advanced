pub mod box_15_1;
pub mod deref_trait_15_2;
use crate::box_15_1::*;
use crate::deref_trait_15_2::*;
fn main() {
    box_15_1::box_create();
    box_15_1::enable_recursive();
    deref_trait_15_2::derefop();
    deref_trait_15_2::ud_sp();
    deref_trait_15_2::ud_sp_deref();
    let name="Anaina".to_string();
    deref_trait_15_2::coercion(&name);
    deref_trait_15_2::noo_coercion();
}