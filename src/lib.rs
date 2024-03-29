
use csaf::Csaf;
use wasm_bindgen::prelude::*;

pub mod verification;

#[wasm_bindgen]
pub fn verification(){

    let csaf: Csaf =
        serde_json::from_str(include_str!("../test-data/rhsa-2023_6583.json"))
            .expect("example data must parse");
    let mut results = vec![];


    results.append(&mut crate::verification::check::vex::check_vulnerabilities_product_status(&csaf));
    results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));
    results.append(&mut crate::verification::check::vex::check_branches_relationships_product_match(&csaf));
    results.append(&mut crate::verification::check::vex::check_vulnerabilities_cve_ids(&csaf));
    results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf));

    let csaf_big: Csaf =
        serde_json::from_str(include_str!("../test-data/rhsa-2023_6818.json"))
            .expect("example data must parse");


    results.append(&mut crate::verification::check::vex::check_vulnerabilities_product_status(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_branches_relationships_product_match(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_vulnerabilities_cve_ids(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf_big));


    let csaf_big: Csaf =
        serde_json::from_str(include_str!("../test-data/rhsa-2023_2097.json"))
            .expect("example data must parse");


    results.append(&mut crate::verification::check::vex::check_vulnerabilities_product_status(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_branches_relationships_product_match(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_vulnerabilities_cve_ids(&csaf_big));
    results.append(&mut crate::verification::check::vex::check_all_products_v11ies_exits_in_product_tree(&csaf_big));
}
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use super::*;
    use log::warn;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_verification() {
        console_log::init_with_level(log::Level::Trace).expect("cannot initialize log");
        let performance = web_sys::window().unwrap().performance().unwrap();
        let start_time = performance.now();


        verification();
        let end_time = performance.now();
        let duration = end_time - start_time;
        warn!("results = {:?}", duration);
    }

}


