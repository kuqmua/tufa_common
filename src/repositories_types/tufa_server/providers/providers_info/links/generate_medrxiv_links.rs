pub fn generate_medrxiv_links(medrxiv_names: Vec<String>) -> Vec<String> {
    //example http://connect.medrxiv.org/medrxiv_xml.php?subject=Addiction_Medicine
    medrxiv_names
        .iter()
        .map(|name| format!("http://connect.medrxiv.org/medrxiv_xml.php?subject={name}"))
        .collect()
}
