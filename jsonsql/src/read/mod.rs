use mysql::{Pool, QueryResult};
use json;

/*******************************************************/
/***************** Support Functions ******************/
/*****************************************************/

pub fn get_col_names(query_result: &QueryResult) -> Vec<String> {

    let column_hash_map = query_result.column_indexes();
    let mut col_tuple_vec: Vec<(String, usize)> = Vec::new();
    let mut col_name_vec: Vec<String> = Vec::new();

    for (name, idx) in column_hash_map.iter() {
        col_tuple_vec.push( (name.to_string(), *idx) )
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

        format!("SELECT * FROM `{}` WHERE `{}`='{}'", table, search_key, search_value)
}

pub fn make_get_statement_2(
    search_cond_one: (&str, &str),
    search_cond_two: (&str, &str),
    table: &str,) -> String {

        format!("SELECT * FROM `{}` WHERE `{}`='{}' AND `{}`='{}'",
        table, search_cond_one.0, search_cond_one.1, search_cond_two.0, search_cond_two.1)
}

pub fn get_by_raw(
    sql: String,
    pool: &mut Pool,) -> Result<String, String> {

        let mut conn = pool.get_conn().unwrap();

        let mut return_array = json::JsonValue::new_array();
        let mut all_row_values: Vec<Vec<String>> = Vec::new();
        let mut conn_error: String = String::from("");

        conn.query(sql)
        .map_err(|err| conn_error = err.to_string())
        .map(|query_result| {
            let col_name_vec: Vec<String> = get_col_names(&query_result);

            for row in query_result {
                let unwrapped = row.unwrap().unwrap();
                let mut row_returns: Vec<String> = Vec::new();

                for value in unwrapped {
                    row_returns.push(value.into_str())
                }

                all_row_values.push(row_returns);
            }

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

pub fn get_by_param(
    search_key: &str,
    search_value: &str,
    table: &str,
    pool: &mut Pool,) -> Result<String, String> {

        let sql: String = make_get_statement(search_key, search_value, table);
        get_by_raw(sql, pool)
}

pub fn get_by_two_params(
    search_cond_one: (&str, &str),
    search_cond_two: (&str, &str),
    table: &str,
    pool: &mut Pool,) -> Result<String, String> {

        let sql: String = make_get_statement_2(search_cond_one, search_cond_two, table);
        get_by_raw(sql, pool)
}

pub fn get_by_id(
  search_value: &str,
  table: &str,
  pool: &mut Pool,) -> Result<String, String> {

      get_by_param("id", search_value, table, pool)
}
