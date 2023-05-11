pub fn generate_biorxiv_links(biorxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.biorxiv.org/biorxiv_xml.php?subject=animal_behavior_and_cognition
    biorxiv_names
        .iter()
        .map(|name| format!("http://connect.biorxiv.org/biorxiv_xml.php?subject={name}"))
        .collect()
}
