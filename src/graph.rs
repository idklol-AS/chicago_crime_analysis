
use std::collections::HashMap;
use crate::read_dataset::crime_dataset::Crime;
    pub fn create_graph(crime_list:&Vec<Crime>) -> Vec<Vec<usize>> {
        let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; crime_list.len()];
        for i in 0..crime_list.len(){
            for j in i+1..crime_list.len(){
               let a = &crime_list[i];
               let b = &crime_list[j];
                if a.ward == b.ward{
                    adjacency_list[i].push(j);
                    adjacency_list[j].push(i);
                }
            }
        }
        adjacency_list
    }
    pub fn find_centrality(crime_list:&Vec<Crime>,adj_list: &Vec<Vec<usize>>) -> Vec<(usize,usize)> {
        let mut nodes:Vec<(usize,usize)> = adj_list.iter().enumerate().map(|(i ,neighbors)| (i,neighbors.len())).collect();
        nodes.sort_by(|a,b| b.1.cmp(&a.1));
        println!("Top 5 most connected crimes (degree centrality):");
        for (index, degree) in nodes.iter().take(5) {
            let crime = &crime_list[*index];
            println!(
                "Crime Case Number: {}, Ward: {}, Degree: {}",
                crime.case_number, crime.ward, degree
            );
    }
    nodes

    }
    pub fn group_by_area(crime_list:&Vec<Crime>,centrality:&Vec<(usize,usize)>){
        let mut area_group:HashMap<usize, (usize,usize)> = HashMap::new();
        for i in 0..crime_list.len(){
            let area = crime_list[i].ward;
            let degrees = centrality[i].1;
            let entry = area_group.entry(area).or_insert((0,0));
            entry.0 += degrees;
            entry.1 += 1;
        }
        let mut sorted:Vec<(usize, f64, usize)> = area_group.into_iter().map(|(area, (degrees, count))| (area,(degrees as f64 / count as f64), count)).collect();
        sorted.sort_by(|a, b| b.2.cmp(&a.2));
        println!("\nTop 5 community areas by number of crimes:");
        for (area, count, avg_degree) in sorted.iter().take(5) {
            println!(
                "Area {} → {} crimes, avg degree {:.2}",
                area, count, avg_degree
            );
        }
    }
    pub fn ward_with_iucr_labels(crime_list:&Vec<Crime>,iucr_labels:HashMap<String, (String, String)>){
        let mut ward_map:HashMap<usize, HashMap<String,usize>> = HashMap::new();
        for crime in crime_list {
            let ward = crime.ward;
            let iucr = crime.iucr.clone();

            let iucr_count = ward_map.entry(ward).or_insert_with(HashMap::new);
            *iucr_count.entry(iucr).or_insert(0) += 1;
        }
        for (ward,iucrs) in ward_map{
            if let Some((iucr, count)) = iucrs.into_iter().max_by_key(|(_, c)| *c) {
                let (primary, secondary) = iucr_labels
                    .get(&iucr)
                    .cloned()
                    .unwrap_or(("UNKNOWN".to_string(), "UNKNOWN".to_string()));
    
                println!(
                    "Ward {:>2}: IUCR {} → {} ({}) – {} reports",
                    ward, iucr, primary, secondary, count
                );
            }
        }
    }
