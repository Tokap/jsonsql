mod conntest;
mod error;

use mysql;
use json;
use std::str::FromStr;

use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use self::conntest::tcp_connect_with_timeout;


/*******************************************************/
/************** Creating a Pool Connection ************/
/*****************************************************/

pub type Pool = mysql::Pool;

// Pass empty str if no option
pub fn build_basic_pool(
    hostname: &str,
    db_name: &str,
    user: &str,
    password: &str,
    port: u16) -> Pool {

        let mut options = mysql::OptsBuilder::new();

        if hostname != "" {options.ip_or_hostname(Some(hostname));}
        if port != 0 {options.tcp_port(port);}
        if db_name != "" {options.db_name(Some(db_name));}
        if user != "" {options.user(Some(user));}
        if password != "" {options.pass(Some(password));}

        let pool = Pool::new(options);
        pool.unwrap()
}

pub fn build_basic_pool_with_tcp_test(
    hostname: &str,
    db_name: &str,
    user: &str,
    password: &str,
    port: u16,
    connection_timeout: u64) -> Result<Pool, String> {

        let tcp_ints: Vec<u8> = hostname.clone().split(".").map(|x| x.parse::<u8>().unwrap()).collect();
        if tcp_ints.len() != 4 { return Err( String::from("Invalid tcp address: wrong number of integer breaks")) }

        // Connection Attempt:
        let five_seconds = Duration::new(connection_timeout, 0);
        let ip_address = IpAddr::V4(Ipv4Addr::new(tcp_ints[0], tcp_ints[1], tcp_ints[2], tcp_ints[3]));
        let socket = SocketAddr::new(ip_address, 20133);
        let connect_attempt = tcp_connect_with_timeout(socket, five_seconds);

        if connect_attempt.is_err() == false {
            // If test is Ok, Build Pool Connection
            let mut options = mysql::OptsBuilder::new();

            if hostname != "" {options.ip_or_hostname(Some(hostname));}
            if port != 0 {options.tcp_port(port);}
            if db_name != "" {options.db_name(Some(db_name));}
            if user != "" {options.user(Some(user));}
            if password != "" {options.pass(Some(password));}

            let pool = Pool::new(options);
            Ok( pool.unwrap() )
        }

        else { Err(String::from("TCP Pool Connection Invalid. Review credentials.")) }
}

pub fn build_pool_json(opts: String) -> Pool {

    let json_opts = json::parse(&opts).unwrap();
    let mut pool_opts = mysql::OptsBuilder::new();

    let hostname: String = json_opts["hostname"].to_string();
    let port: String     = json_opts["port"].to_string();
    let db: String       = json_opts["db"].to_string();
    let user: String     = json_opts["user"].to_string();
    let password: String = json_opts["password"].to_string();
    let socket: String   = json_opts["socket"].to_string();

    if hostname != "null" && hostname != "" { pool_opts.ip_or_hostname(Some(hostname)); }
    if port != "null" && port != ""         { pool_opts.tcp_port(u16::from_str(&port).unwrap()); }
    if db != "null" && db != ""             { pool_opts.db_name(Some(db)); }
    if user != "null" && user != ""         { pool_opts.user(Some(user)); }
    if password != "null" && password != "" { pool_opts.pass(Some(password)); }
    if socket != "null" && socket != ""     { pool_opts.socket(Some(socket)); }

    let pool = Pool::new(pool_opts);
    pool.unwrap()
}

pub fn connection_is_active(p: Pool) -> bool {

    match p.try_get_conn(50000) {
        Ok(_) => true,
        Err(_) => false,
    }
}
