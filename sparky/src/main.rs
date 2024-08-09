use crate::dataframe::CSVFile;
use crate::dataframe::CellType;
use crate::dataframe::Column;
use crate::dataframe::ColumnType;
use crate::dataframe::DataFrame;
use crate::dataframe::Row;

mod dataframe;

fn main() {
    println!("# Sparky");
    println!("## Data ETL Tool in Rust\n\n");

    let csv = CSVFile(String::from("./sample.csv"));
    println!("### Reading from {} and inferring types.\n", csv.0);
    let df_csv = DataFrame::from(csv);
    println!("{}\n\n", df_csv);

    println!("### Creating a dataframe with strings\n",);
    let rows = Some(vec![
        Row::new(
            vec!["Hello", "There", "."]
                .into_iter()
                .map(|v| CellType::Text(String::from(v)))
                .collect(),
        ),
        Row::new(
            vec!["Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.", "Kenobi!", "New"]
                .into_iter()
                .map(|v| CellType::Text(String::from(v)))
                .collect(),
        ),
    ]);
    let columns = vec![
        Column {
            data_type: ColumnType::Text,
            name: String::from("First Name"),
        },
        Column {
            data_type: ColumnType::Float,
            name: String::from("Last Name"),
        },
        Column {
            data_type: ColumnType::Float,
            name: String::from("Email"),
        },
    ];
    let df3: DataFrame = DataFrame::new(rows, columns);
    println!("{}\n\n", df3);

    println!("### Creating new weather dataframe with three float fields\n",);
    let columns = vec![
        Column {
            data_type: ColumnType::Float,
            name: String::from("Temp (F)"),
        },
        Column {
            data_type: ColumnType::Float,
            name: String::from("Temp (C)"),
        },
        Column {
            data_type: ColumnType::Float,
            name: String::from("Humidity"),
        },
    ];
    let rows = Some(vec![
        Row::new(vec![
            CellType::Float(10109876543.01234),
            CellType::Float(4.01),
            CellType::Float(4.02),
        ]),
        Row::new(vec![
            CellType::Float(0.0),
            CellType::Float(1.0),
            CellType::Float(4.02),
        ]),
    ]);
    let mut df1: DataFrame = DataFrame::new(rows, columns);
    println!("{}\n\n", df1);

    println!("### Creating another weather dataframe with three float fields\n",);
    let columns = vec![
        Column {
            data_type: ColumnType::Float,
            name: String::from("Temp (F)"),
        },
        Column {
            data_type: ColumnType::Float,
            name: String::from("Temp (C)"),
        },
        Column {
            data_type: ColumnType::Float,
            name: String::from("Humidity"),
        },
    ];
    let rows = Some(vec![
        Row::new(vec![
            CellType::Float(4.1),
            CellType::Float(5.1),
            CellType::Float(6.1),
        ]),
        Row::new(vec![
            CellType::Float(0.1),
            CellType::Float(1.1),
            CellType::Float(2.1),
        ]),
    ]);
    let mut df2: DataFrame = DataFrame::new(rows, columns);
    println!("{}\n\n", df2);

    println!("### Performing a union on the weather dataframes\n",);
    let df_union_all = df1.union_all(&mut df2);
    match df_union_all {
        Ok(df) => println!("{}\n\n", df),
        Err(e) => println!("Error occurred: {e}"),
    }
}
