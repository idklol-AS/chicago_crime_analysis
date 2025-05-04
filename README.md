Project Overview
Goal: What question are you answering?
The question that I am answering is what are the most common occurring crimes based on IUCR labels in the 50 wards of Chicago and the amount of reports regarding the most common occurring crime in the past year.
Dataset: Source, size (link if too large for GitHub).
https://catalog.data.gov/dataset/crimes-one-year-prior-to-present
https://data.cityofchicago.org/Public-Safety/Chicago-Police-Department-Illinois-Uniform-Crime-R/c7ck-438e/about_data?no_mobile=true

B. Data Processing
How you loaded it into Rust.
I used the csv crate to load the data into Rust. I iterated over each row in the csv file and extracted fields from each row which were case_number, ward, description, date, and iucr and put them into a struct called Crime which then I pushed the struct I created into a vector with all of the crimes recorded.
Any cleaning or transformations applied.
I filtered rows that had missing or invalid ward values. I normalized the IUCR by making sure that each IUCR was 4 digits long to avoid mismatches.I skipped rows that had a length less than the columns that were in the dataset.

C. Code Structure
Modules
Purpose of each and rationale for organization.
Read_dataset module read and parsed through all the data in the crime dataset and defined the Crime struct for each crime recorded. It also mapped the IUCR codes to the primary and secondary descriptions that it corresponds to.
Graph module builds a graph of crimes based on IUCR and Ward and calculates the centrality. 
The main module calls functions from other modules and reads, analyzes, and prints the results from the dataset it is given.
Key Functions & Types (Structs, Enums, Traits, etc)

The struct Crime represents one crime incident from the dataset. It had fields called  case_number, ward, description, date, and iucr to describe the crime more clearly. It is used to analyze and construct the graphs. 
Fn read_data loads and filters the dataset from a CSV file. The input is the path of the dataset CSV file. THe output is a vector with struct Crime records. csv::Reader is used to iterate over rows, it skips rows with no or incorrect ward values.
Fn read_iucr maps IUCR codes to their primary and secondary descriptions it represents. The input is the path of the IUCR CSV file. The output is a Hashmap that connects IUCR codes with their primary and secondary descriptions as a tuple. The function skips the header, trims each field and stores results in the HashMap.
Fn create_graph builds an adjacency list from the crime_list we created and connects them by their shared ward. The input was the vector with all crimes committed. The output is an adjacency list graph. For each pair of crimes, it checks if they share the same ward, and if they do, they add an undirected edge in between them. 
Fn find_centrality calculates the degree for each crime node. The input is the crime list and the graph created from fn create_graph. The output is a vector of tuples with the crime index and degree. It uses .enumerate().map() to count the edges per node. It also sorts the node from largest degree to smallest degree and prints out the 5 most connected nodes with their case number, ward, and degree.
Fn group_by_area, groups crimes by ward and computes the average degree centrality per ward. The input is the crime_list and the output from the fn find_centrality function. The output of the function is it prints the top 5 wards by number of crimes along with their average degree. It iterates through all crimes and gets the degree and count per ward. It calculates the average degree per ward and sorts the total number of crimes from largest to smallest. 
Fn ward_with_iucr_labels identifies the most commonly occurring IUCR code in each ward and displays its label and count. The input is the crime_list and the iucr_labels created from read_iucr. The output prints out the most common IUCR per ward, with its description and report count. The logic is that I built a nested map of the ward and for each ward, it finds the IUCR with the highest frequency and then looks up the IUCR descriptions. 
Main Workflow


At a glance, how modules/functions interact to produce your results.
Main.rs calls read_data() to load and clean the crime dataset. The IUCR label map is loaded from the read_iucr function. Create_graph() makes connections between crimes that share the same ward from the read_data() output. The find_centrality() ranks crimes by the amount of connections it has. Group_by_area shows the ward with the most crimes committed and ward_with_iucr_labels work with other function outputs to get the most commonly occurring IUCR in each ward.


D. Tests
cargo test output (paste logs or provide screenshots).
running 3 tests
test crime_dataset::tests::test_iucr_to_four_digits ... ok
test crime_dataset::tests::test_valid_crime_struct ... ok
test graph::tests::test_graph_connections_by_iucr ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


For each test: what it checks and why it matters.
The first test matters because it makes sure that IUCR is 4 digits long and changes the IUCR if it doesn’t have 4 digits. The second test matters because it checks to make sure that the Crime structure stores valid values. The third test matters because it makes sure that crimes committed in the same ward are connected in the graph.


E. Results
All program outputs (screenshots or pasted).

Top 5 wards by number of crimes:
Area 27 → 6177.540952707333 crimes, avg degree 11672
Area 28 → 6144.95270086246 crimes, avg degree 11015
Area 6 → 6162.749111661462 crimes, avg degree 9287
Area 42 → 6217.434342284934 crimes, avg degree 8788
Area 24 → 6085.988981980948 crimes, avg degree 8713

 The most commonly occurring IUCR in each ward.
Ward 34: IUCR 0860 → THEFT (RETAIL THEFT) – 1392 reports
Ward 45: IUCR 0860 → THEFT (RETAIL THEFT) – 358 reports
Ward 22: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 306 reports
Ward 19: IUCR 0860 → THEFT (RETAIL THEFT) – 240 reports
Ward  5: IUCR 0820 → THEFT ($500 AND UNDER) – 469 reports
Ward 26: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 370 reports
Ward 43: IUCR 0860 → THEFT (RETAIL THEFT) – 419 reports
Ward 27: IUCR 0810 → THEFT (OVER $500) – 1000 reports
Ward 16: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 874 reports
Ward  4: IUCR 0820 → THEFT ($500 AND UNDER) – 628 reports
Ward 29: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 653 reports
Ward 31: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 308 reports
Ward  9: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 733 reports
Ward 11: IUCR 0820 → THEFT ($500 AND UNDER) – 275 reports
Ward 25: IUCR 0460 → BATTERY (SIMPLE) – 354 reports
Ward 14: IUCR 0860 → THEFT (RETAIL THEFT) – 491 reports
Ward 35: IUCR 0860 → THEFT (RETAIL THEFT) – 524 reports
Ward 40: IUCR 0810 → THEFT (OVER $500) – 254 reports
Ward 10: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 497 reports
Ward 44: IUCR 0860 → THEFT (RETAIL THEFT) – 616 reports
Ward  2: IUCR 0860 → THEFT (RETAIL THEFT) – 457 reports
Ward 49: IUCR 0860 → THEFT (RETAIL THEFT) – 390 reports
Ward 13: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 268 reports
Ward  6: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 883 reports
Ward 21: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 638 reports
Ward 33: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 271 reports
Ward  7: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 754 reports
Ward 47: IUCR 0810 → THEFT (OVER $500) – 355 reports
Ward  3: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 581 reports
Ward 17: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 943 reports
Ward 48: IUCR 0460 → BATTERY (SIMPLE) – 324 reports
Ward 36: IUCR 0860 → THEFT (RETAIL THEFT) – 412 reports
Ward 20: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 880 reports
Ward 12: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 306 reports
Ward 50: IUCR 0860 → THEFT (RETAIL THEFT) – 248 reports
Ward 38: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 206 reports
Ward  1: IUCR 0810 → THEFT (OVER $500) – 492 reports
Ward 15: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 444 reports
Ward 32: IUCR 0860 → THEFT (RETAIL THEFT) – 764 reports
Ward 42: IUCR 0810 → THEFT (OVER $500) – 964 reports
Ward 37: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 792 reports
Ward 39: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 215 reports
Ward 18: IUCR 0910 → MOTOR VEHICLE THEFT (AUTOMOBILE) – 338 reports
Ward 30: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 277 reports
Ward 23: IUCR 0910 → MOTOR VEHICLE THEFT (AUTOMOBILE) – 290 reports
Ward 41: IUCR 0810 → THEFT (OVER $500) – 368 reports
Ward 24: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 865 reports
Ward 28: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 913 reports
Ward 46: IUCR 0460 → BATTERY (SIMPLE) – 456 reports
Ward  8: IUCR 0486 → BATTERY (DOMESTIC BATTERY SIMPLE) – 697 reports



Interpretation in project context (no need for "groundbreaking” results).
The results for top five wards by number of crimes show the wards with the highest volume of crimes based on incident counts in the past year and the high average degree value shows that there is strong connection in crime patterns in these areas meaning they share the same attributes.
The result for most commonly occurring IUCR per ward shows that certain types of crimes dominate such as Retail Theft, Domestic Battery, and Theft over $500. 


F. Usage Instructions
How to build and run your code.
Description of any command-line arguments or user interaction in the terminal.


Include expected runtime especially if your project takes a long time to run.
To build my project I created a function that would read the Chicago crime dataset CSV file and would create a graph connecting crimes based on the ward they were committed in. I then analyzed the centrality of the crimes and grouped the crimes by ward. The runtime for my project was 1 minute with cargo run --release.
 
