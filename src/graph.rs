
use std::collections::HashMap;
use crate::crime_dataset::Crime;
//The purpose of this module is to create a graph from the dataset that we read already. 
//It also helps with analyzing the data by finding the centrality and groups them by the neighborhood ward and returns the most common crime tha happens in each ward.

//This function creates a graph from the data that was organized from the crime_dataset module.
//The input is the crime_list that was created from the crime_dataset module. 
//The output is a vector of vectors which is an adjacency list that connect crimes based on the ward they are committed in.
    pub fn create_graph(crime_list:&Vec<Crime>) -> Vec<Vec<usize>> {
        let mut adjacency_list: Vec<Vec<usize>> = vec![vec![]; crime_list.len()];
        //This iterates over the crime_list
        for i in 0..crime_list.len(){
            //This iterates over the crime_list starting at the index of i+1.
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
//This function finds the centrality of the crimes committed.
//The input is the crime_list created from the crime_dataset module and the adjacency list that is created from the function above.
// The output is a vector that creates nodes from crimes and brings out the crime with the most connected crimes based on ward by sorting by which one has the highest degree.
    pub fn find_centrality(crime_list:&Vec<Crime>,adj_list: &Vec<Vec<usize>>) -> Vec<(usize,usize)> {
        let mut nodes:Vec<(usize,usize)> = adj_list.iter().enumerate().map(|(i ,neighbors)| (i,neighbors.len())).collect();
        nodes.sort_by(|a,b| b.1.cmp(&a.1));
        println!("Top 5 most connected crimes (degree centrality):");
        //This iterates over the nodes list that is sorted from most connected to least connected and only takes the first five crimes that are most connected.
        for (index, degree) in nodes.iter().take(5) {
            let crime = &crime_list[*index];
            println!(
                "Crime Case Number: {}, Ward: {}, Degree: {}",
                crime.case_number, crime.ward, degree
            );
    }
    nodes

    }
    //The purpose of this function is to the return the ward with the most crimes committed. 
    //The input for this function is the crime_list and the centrality vectors of the crime_list that were calculated from the function above.
    pub fn group_by_area(crime_list:&Vec<Crime>,centrality:&Vec<(usize,usize)>){
        let mut area_group:HashMap<usize, (usize,usize)> = HashMap::new();
        //This iterates over the crime_list created from the dataset.
        for i in 0..crime_list.len(){
            // I created a hashmap that has the key has the ward and degrees as the value stored in it.
            let area = crime_list[i].ward;
            let degrees = centrality[i].1;
            let entry = area_group.entry(area).or_insert((0,0));
            //This tracks the amount of crimes committed in an area when it is being iterated and also adds the degrees to the area degree.
            entry.0 += degrees;
            entry.1 += 1;
        }
        //This sorts the keys by the wards with the highest number of crimes.
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
    //This function calculates the most common crime to occur in each ward and prints out the most common crime along with the ward. 
    //The input for this function is the crime_list and iucr_labels that were created from the crime_dataset module.
    pub fn ward_with_iucr_labels(crime_list:&Vec<Crime>,iucr_labels:HashMap<String, (String, String)>){
        let mut ward_map:HashMap<usize, HashMap<String,usize>> = HashMap::new();
        //The function iterates over each crime is inserted into a hashmap where the key is the ward, and the value is another HashMap that stores the IUCR label and the amount of times that specific IUCR label is committed in the ward.
        for crime in crime_list {
            let ward = crime.ward;
            let iucr = crime.iucr.clone();

            let iucr_count = ward_map.entry(ward).or_insert_with(HashMap::new);
            *iucr_count.entry(iucr).or_insert(0) += 1;
        }
        //This for loop prints out each ward where it's the most common crime occurred in said ward and the amount of reports correlating to said crime label.
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
