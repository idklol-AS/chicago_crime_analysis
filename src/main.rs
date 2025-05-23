mod crime_dataset;
mod graph;
use crime_dataset::{read_data, read_iucr};
use graph::{create_graph,find_centrality,group_by_area, ward_with_iucr_labels};
//The main function runs all the code with the datasets.
fn main() {
    let (crime_list, area_map) = read_data("src/Crimes_-_One_year_prior_to_present.csv");
    let iucr_list = read_iucr("src/Chicago_Police_Department_-_Illinois_Uniform_Crime_Reporting__IUCR__Codes_20250501.csv");
    let adj_list = create_graph(&crime_list);
    let degrees = find_centrality(&crime_list, &adj_list);
    group_by_area(&crime_list, &degrees);
    ward_with_iucr_labels(&crime_list, iucr_list);
}
