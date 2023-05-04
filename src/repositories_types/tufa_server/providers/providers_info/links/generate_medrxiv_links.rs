pub fn generate_medrxiv_links(medrxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine
    medrxiv_names
        .iter()
        .map(|name| format!(
            "{}{name}",
            crate::global_variables::hardcode::MEDRXIV_LINK_FIRST_PART
        ))
        .collect()
}
