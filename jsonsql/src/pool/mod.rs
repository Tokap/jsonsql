use mysql;

/*******************************************************/
/************** Creating a Pool Connection ************/
/*****************************************************/

pub type Pool = mysql::Pool;

// expand to allow for more options
#[allow(dead_code)]
pub fn build_pool(
    hostname: &str,
    db_name: &str,
    user: &str,
    port: u16) -> Pool {

        let mut builder = mysql::OptsBuilder::new();
        builder
            .ip_or_hostname(Some(hostname))
            .tcp_port(port)
            .db_name(Some(db_name))
            .user(Some(user));
        let pool = Pool::new(builder);
        pool.unwrap()
}

#[allow(dead_code)]
pub fn test_and_output_connection(p: Pool) -> bool {

    match p.try_get_conn(50000) {
        Ok(_) => true,
        Err(_) => false,
    }
}
