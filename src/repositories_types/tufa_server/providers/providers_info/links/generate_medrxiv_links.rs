use crate::global_variables::hardcode::MEDRXIV_LINK_FIRST_PART;

pub fn generate_medrxiv_links(medrxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine
    medrxiv_names
        .iter()
        .map(|name| format!("{MEDRXIV_LINK_FIRST_PART}{name}"))
        .collect()
}
