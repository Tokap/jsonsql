use mysql::{Pool, QueryResult};
use json;

/*******************************************************/
/***************** Support Functions ******************/
/*****************************************************/

#[allow(dead_code)]
pub fn get_col_names(query_result: &QueryResult) -> Vec<String> {

    let column_hash_map = query_result.column_indexes(); // Split col names to hashmap
    let mut col_tuple_vec: Vec<(String, usize)> = Vec::new(); // Vec<(column_name, idx)>
    let mut col_name_vec: Vec<String> = Vec::new(); // Final Vec for return

    for (name, idx) in column_hash_map.iter() {
        col_tuple_vec.push( (name.to_string(), *idx) ) // make tuples w/ col name & index
    }
    col_tuple_vec.sort_by(|a,b| a.1.cmp(&b.1));

    for (column_name, _) in col_tuple_vec {
        col_name_vec.push(column_name);
    }
    col_name_vec
}

/*******************************************************/
/**************** Main Read Function  *****************/
/*****************************************************/

pub fn make_get_statement(
    search_key: &str,
    search_value: &str,
    table: &str,) -> String {

        format!("SELECT * FROM `{}` WHERE `{}`={}", table, search_key, search_value)
}

#[allow(dead_code)]
pub fn get_by_raw(
    sql: String,
    pool: Pool,) -> Result<String, String> {

        let mut conn = pool.get_conn().unwrap();

        let mut return_array = json::JsonValue::new_array();
        let mut all_row_values: Vec<Vec<String>> = Vec::new();
        let mut conn_error: String = String::from("");

        conn.query(sql)
        .map_err(|err| conn_error = err.to_string())
        .map(|query_result| {
            let col_name_vec: Vec<String> = get_col_names(&query_result);

            // Create Vector of Vec<String> holding value on each row w/o keys
            for row in query_result {
                let unwrapped = row.unwrap().unwrap();
                let mut row_returns: Vec<String> = Vec::new();

                for value in unwrapped {
                    row_returns.push(value.into_str())
                }

                all_row_values.push(row_returns);
            }

            // Go through each row's content, assign it a key with col names & create JSON
            for row_contents in all_row_values {

                let mut data_object = json::JsonValue::new_object();
                let col_names: Vec<String> = col_name_vec.clone();

                for i in 0..row_contents.len() {
                    data_object[col_names[i].to_owned()] = row_contents[i].replace("'", "").into();
                }

                return_array.push(data_object);
            }
        });

        match conn_error.len() {
            0   => Ok(return_array.dump()),
            _ => Err(conn_error),
        }
}

//******************************************************/
//*********** Combined Read Functions *****************/
//****************************************************/

#[allow(dead_code)]
pub fn get_by_param(
    search_key: &str,
    search_value: &str,
    table: &str,
    pool: Pool,) -> Result<String, String> {

        let sql: String = make_get_statement(search_key, search_value, table);
        get_by_raw(sql, pool)
}

#[allow(dead_code)]
pub fn get_json_by_id(
  search_value: &str,
  table: &str,
  pool: Pool,) -> Result<String, String> {

      get_by_param("id", search_value, table, pool)
}
