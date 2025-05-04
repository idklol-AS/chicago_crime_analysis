
//The purpose of the module is to load the dataset and read the dataset by adding the data into a crime struct I created and then adding them into a vector to have all of the crimes. 
use csv::Reader;
use std::collections::HashMap;
#[derive(Debug,Clone)]
//This structure represents a crime and the elements involved with it. it stores the case number, the ward the crime was committed in, the description of the crime, the date of the crime, and the IUCR code.
pub struct Crime {
    pub case_number:String,
    pub ward: usize, 
    pub description: String, 
    pub date: String,
    pub iucr:String,
}
//This function reads the data. The input is the path of the dataset. 
//The output of this function is tuple of a vector that has all of the crimes and a Hashmap that organized crimes based on what ward it is committed in. 
pub fn read_data(path: &str) -> (Vec<Crime>, HashMap<usize, Vec<Crime>>) {
    let mut rdr = Reader::from_path(path).unwrap();

    let mut crime_list: Vec<Crime> = Vec::new();
    let mut ward_map: HashMap<usize, Vec<Crime>> = HashMap::new();
// This for loop is looping over each line in the dataset.
    for result in rdr.records() {
        let record = result.unwrap();
// This checks whether the row has 13 entries and if the column that records the IUCR code is in each row otherwise it skips it if the conditions aren't fulfilled.
        if record.len() <= 13 || record[3].is_empty() {
            continue;
        
        }
        let ward = record[10].trim().parse::<usize>().unwrap();
        let crime = Crime {
            case_number: record[0].to_string(),
            ward: ward,
            date: record[1].to_string(),
            description: record[4].to_string(),
            iucr: record[3].to_string(),
        };
        ward_map.entry(crime.ward)
                .or_default()
                .push(crime.clone());
            crime_list.push(crime);
    }
    (crime_list,ward_map)
}
//This function reads another dataset that stores what each IUCR code is. IUCR codes indicate what is the crime that is commmitted and provides more details involving the crime.
//The input is the path of the dataset containing IUCR codes. This outputs a HashMap that contains the IUCR codes as a key and value is a primary and secondary description of the crime that is being represented from IUCR code.
pub fn read_iucr(path:&str) -> HashMap<String, (String, String)>{
    let mut rdr = Reader::from_path(path).unwrap();
    let mut label_map = HashMap::new();
    //This for loop iterates over each row and reads the data.
    for result in rdr.records(){
        //This records the code, the primary description of the crime, and the secondary description of the crime. It also makes sure that the digits of the IUCR is a length of 4 and if not, it adds a 0.
        let record = result.unwrap();
        let mut iucr = record[0].trim().to_string();
        let primary = record[1].trim().to_string();
        let secondary = record[2].trim().to_string();
        if iucr.len() == 3 && iucr.chars().all(|c| c.is_ascii_digit()) {
            iucr = format!("0{}", iucr);
        }
        //This inserts the IUCR code as the key and the value that outputs is the primary and secondary description of the crime from the code.
        label_map.insert(iucr,(primary, secondary));
    }
    label_map
}

