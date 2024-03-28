use csaf::Csaf;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::verification::check::init_verifying_visitor;
use crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree;

pub mod check;

#[wasm_bindgen]
pub fn verification(){
    for v in init_verifying_visitor(){
        let csaf: Csaf =
            serde_json::from_str(include_str!("../../test-data/rhsa-2023_1441.json"))
                .expect("example data must parse");
        let mut results = vec![];
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
        results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
    }
}