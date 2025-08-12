use std::collections::{HashMap};
use std::vec::Vec;
use std::error::Error;
use std::fmt;
use std::process;

#[derive(Debug, Clone)]
enum Types {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug)]
struct DataFrame {
    labels: Vec<String>,
    data: HashMap<String, Vec<Types>>,
}

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

impl DataFrame {
    fn new() -> Self {
        DataFrame {
            labels: Vec::new(),
            data: HashMap::new(),
        }
    }

    fn add_column(&self, colname: String, colval: Vec<Types>) -> DataFrame {
        let mut ans = DataFrame::new();
        ans.labels.extend(self.labels.clone());
        self.data.iter().for_each(|(k, v)| {
            ans.data.insert(k.clone(), v.clone());
        });
    
        if self.labels.is_empty() {
            // No existing data — just add it
            ans.labels.push(colname.clone());
            ans.data.insert(colname, colval);
        } else {
            let reference_col = &self.labels[0];
            let vec = self.data.get(reference_col).expect("Corrupt df");
    
            if colval.len() != vec.len() {
                panic!("Length mismatch");
            }
    
            ans.labels.push(colname.clone());
            ans.data.insert(colname, colval);
        }
    
        ans
    }
    

    fn merge_frame(&self, rightdf: DataFrame) -> DataFrame {       
        if self.labels != rightdf.labels {
            println!("Different Labels");
            panic!();
        } else {
            self.labels.iter().for_each(|i| {
                if let (Some(l), Some(r)) = (self.data.get(i), rightdf.data.get(i)) {
                    match (&l[0], &r[0]) { 
                        (Types::One(_), Types::One(_)) => (),
                        (Types::Two(_), Types::Two(_)) => (),
                        (Types::Three(_), Types::Three(_)) => (),
                        (Types::Four(_), Types::Four(_)) => (),
                        _ => panic!("Different types"),
                    }
                }
            });
            let mut ans = DataFrame::new();
            ans.labels = self.labels.clone();
            self.data.iter().for_each(|(k, v)| { ans.data.insert(k.clone(), v.clone());} );
            rightdf.data.iter().for_each(|(k, v)| { ans.data.get_mut(k).unwrap().extend(v.clone()); } );
            return ans;
        }
    }

    fn restrict_columns(&self, cols: &[String]) -> DataFrame {
        let mut ans = DataFrame::new();
        cols.iter().for_each(|s| {
            if self.labels.contains(s) { ans.labels.push(s.clone()); } else { panic!("Invalid label"); }
        });
        ans.labels.iter().for_each(|s| {
            ans.data.insert(s.clone(), self.data.get(s).unwrap().clone());
        });
        return ans;
    }

    fn filter<F>(&self, column: &str, predicate: F) -> DataFrame where 
        F: Fn(&Types) -> bool {
            let mut ans = DataFrame::new();
            ans.labels = self.labels.clone();
            let col = self.data.get(column).unwrap();
            ans.labels.iter().for_each(|s| { ans.data.insert(s.clone(), Vec::new()); });
            (0..col.len()).for_each(|i| { 
                if predicate(&col[i]){
                    for k in self.labels.iter() { 
                        ans.data.get_mut(k).unwrap().push(self.data.get(k).unwrap()[i].clone()); 
                    }
                }
            });
            return ans;     
    }

    fn column_op<F>(&self, labels: &[String], op: F) -> Vec<Types>
    where
        F: Fn(&Vec<Types>) -> Vec<Types>,
    {
        if labels.len() == 1 {
            let columns = self.data.get(&labels[0]).unwrap();
            return op(columns);
        }
        else {
            let cols: Vec<Types> = labels.iter().flat_map(|label| self.data.get(label).unwrap().clone()).collect();
            return op(&cols);
        }
    }

    fn median(&self, cols: &[String]) -> Vec<Types> {
        let mediancalc = |columns: &Vec<Types>| {
            let mut nums: Vec<f64> = columns
                .iter()
                .filter_map(|v| match v {
                    Types::Three(f) => Some(*f),
                    Types::Four(i) => Some(*i as f64),
                    _ => panic!("Not numeric"),
                })
                .collect();
    
            if nums.is_empty() {
                return vec![];
            }
    
            nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
            let mid = nums.len() / 2;
            let median = if nums.len() % 2 == 0 {
                (nums[mid - 1] + nums[mid]) / 2.0
            } else {
                nums[mid]
            };
    
            vec![Types::Three(median)]
        };

        self.column_op(cols, mediancalc)
    }

    fn sub_columns(&self, cols: &[String]) -> Vec<Types> {
        let subtractcalc = |flat: &Vec<Types>| {
            let num_rows = self.data.get(&cols[0]).unwrap().len();
            let mut result = Vec::with_capacity(num_rows);
    
            for i in 0..num_rows {
                let a = &flat[i];
                let b = &flat[i + num_rows];
    
                let val = match (a, b) {
                    (Types::Three(x), Types::Three(y)) => Types::Three(x - y),
                    (Types::Four(x), Types::Four(y)) => Types::Four(x - y),
                    (Types::Three(x), Types::Four(y)) => Types::Three(x - *y as f64),
                    (Types::Four(x), Types::Three(y)) => Types::Three(*x as f64 - y),
                    _ => panic!("Unsupported types in sub_columns"),
                };
    
                result.push(val);
            }
    
            result
        };
    
        self.column_op(cols, subtractcalc)
    }
    

    fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        let mut first_row = true;
        for result in rdr.records() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let r = result.unwrap();
            // println!("row: {:?}", r);
            let mut row: Vec<Types> = vec![];
            let mut labels: Vec<String> = vec![];
            if first_row {
                for elem in r.iter() {
                    labels.push(elem.to_string())
                }
                self.labels = labels.clone();
                for label in &labels {
                    self.data.insert(label.clone(), Vec::new());
                }
                first_row = false;
                continue;
            }
            for (i, elem) in r.iter().enumerate() {
                match types[i] {
                    1 => row.push(Types::One(elem.to_string())),
                    2 => row.push(Types::Two(elem.parse::<bool>().unwrap())),
                    3 => row.push(Types::Three(elem.parse::<f64>().unwrap())),
                    4 => row.push(Types::Four(elem.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
            // Put the data into the dataframe
            for i in 0..row.len() {
                self.data.entry(self.labels[i].clone()).and_modify(|x| x.push(row[i].clone())).or_insert(vec![row[i].clone()]);
            }
        }
        Ok(())
    }

    fn print(&self) {
        println!("{:?}", self.labels);
        println!("{:?}", self.data);
    }
}

fn main() {
    println!("Getting df1");
    let mut df1 = DataFrame::new();
    df1.read_csv("/opt/app-root/src/hw8/pizzaone.csv", &vec![1, 4, 3, 4, 4, 2]).unwrap();
    df1.print();

    println!("Getting df2");
    let mut df2 = DataFrame::new();
    df2.read_csv("/opt/app-root/src/hw8/pizzatwo.csv", &vec![1, 4, 3, 4, 4, 2]).unwrap();
    df2.print();

    let hall_of_fame = vec![
        Types::Two(true),
        Types::Two(false),
        Types::Two(false),
        Types::Two(false),
        Types::Two(true),
    ];

    let hall_of_blegh = vec![
        Types::Two(true),
    ];

    println!("Add Column");
    let df_add_column = df1.add_column("Hall of Fame".to_string(), hall_of_fame);
    df_add_column.print();

    // let df_add_column_panic = df1.add_column("Hall of Fame".to_string(), hall_of_blegh);
    // df_add_column_panic.print();

    println!("Merge df");
    let df_merged = df1.merge_frame(df2);
    df_merged.print();

    println!("Restrict");
    let df_restricted = df1.restrict_columns(&vec![
        "Name".to_string(),
        "TotalPoints".to_string(),
    ]);
    df_restricted.print();

    let customized_filter = |v: &Types| -> bool { match v {
        Types::Three(f) => *f > 25.0,
        _=> false,
    }};
    
    let customized = |v: &Vec<Types>| -> Vec<Types> {
        v.iter()
         .map(|i| match i {
             Types::Three(f) => Types::Two(*f > 25.0),
             _ => Types::Two(false),
         })
         .collect()
    };
    

    println!("Filter");
    let df_filter = df1.filter("PPG", |v| match v {
        Types::Three(f) => *f > 25.0,
        _ => false,
    });
    df_filter.print();

    let df_test_filter = df1.filter("PPG", customized_filter);
    df_test_filter.print();

    println!("Test column_op()");
    let test_columnop = df1.column_op(&["PPG".to_string()], customized);
    println!("{:?}", test_columnop);

    println!("Median");
    let result = df1.median(&vec!["PPG".to_string()]);
    println!("{:?}", result);

    println!("Subtraction");
    let result = df1.sub_columns(&vec!["TotalPoints".to_string(), "YearBorn".to_string()]);
    println!("{:?}", result);
}
