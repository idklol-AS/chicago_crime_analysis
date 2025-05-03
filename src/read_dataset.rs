pub mod crime_dataset {
use csv::Reader;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub struct Crime {
    pub case_number:String,
    pub ward: usize, 
    pub description: String, 
    pub date: String,
    pub iucr:String,
}
pub fn read_data(path: &str) -> (Vec<Crime>, HashMap<usize, Vec<Crime>>) {
    let mut rdr = Reader::from_path(path).unwrap();

    let mut crime_list: Vec<Crime> = Vec::new();
    let mut ward_map: HashMap<usize, Vec<Crime>> = HashMap::new();
    let mut skipped = 0;

    for result in rdr.records() {
        let record = result.unwrap();

        if record.len() <= 13 || record[3].is_empty() {
            skipped += 1;
            continue;
        
        }
        let ward = match record[10].trim().parse::<usize>() {
            Ok(w) => w,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };
        let crime = Crime {
            case_number: record[0].to_string(),
            date: record[1].to_string(),
            ward: ward,
            description: record[4].to_string(),
            iucr: record[3].to_string(),
        };
        ward_map.entry(crime.ward)
                .or_default()
                .push(crime.clone());
            crime_list.push(crime);
    }
    println!("Amount of rows skipped:{}", skipped);
    (crime_list,ward_map)
}
pub fn read_iucr(path:&str) -> HashMap<String, (String, String)>{
    let mut rdr = Reader::from_path(path).unwrap();
    let mut label_map = HashMap::new();
    for result in rdr.records(){
        let record = result.unwrap();
        let mut iucr = record[0].trim().to_string();
        let primary = record[1].trim().to_string();
        let secondary = record[2].trim().to_string();
        if iucr.len() == 3 && iucr.chars().all(|c| c.is_ascii_digit()) {
            iucr = format!("0{}", iucr);
        }
        label_map.insert(iucr,(primary, secondary));
    }
    label_map
}
}

