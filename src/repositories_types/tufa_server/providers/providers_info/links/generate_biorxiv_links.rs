use crate::global_variables::hardcode::BIORXIV_LINK_FIRST_PART;

pub fn generate_biorxiv_links(biorxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition
    biorxiv_names
        .iter()
        .map(|name| format!("{BIORXIV_LINK_FIRST_PART}{name}"))
        .collect()
}
