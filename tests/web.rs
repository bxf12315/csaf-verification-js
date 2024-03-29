//
// use csaf::Csaf;
// use csaf_verification_js::verification::check::vex::check_all_products_v11ies_exits_in_product_tree;
// // use crate::vex::check_all_products_v11ies_exits_in_product_tree;
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// pub fn verification(){
//     let csaf: Csaf =
//         serde_json::from_str(include_str!("../test-data/rhsa-2023_1441.json"))
//             .expect("example data must parse");
//     let mut results = vec![];
//
//
//     results.append(&mut check_all_products_v11ies_exits_in_product_tree(&csaf));
//     // results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
//     // results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
//     // results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
//     // results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
//     // results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
//     // results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
//     log::warn!("{:?}", results);
// }
// #[cfg(test)]
// mod tests {
//     use wasm_bindgen_test::*;
//     use super::*;
//
//     wasm_bindgen_test_configure!(run_in_browser);
//
//     // #[wasm_bindgen_test]
//
//     fn test_verification() {
//         verification();
//     }
// }
//
//
