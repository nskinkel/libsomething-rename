use std::net::{Ipv4Addr, Ipv6Addr};
use time::{Tm};

#[derive(Debug)]
pub enum Flag {
    Authority,
    BadExit,
    Exit,
    Fast,
    Guard,
    HSDir,
    Named,
    Stable,
    Running,
    Valid,
    V2Dir,
}

#[derive(Debug)]
pub struct Preamble;

#[derive(Debug)]
pub struct Authority;

#[derive(Debug)]
pub struct Entry {
    /* 'r' line */
    pub nickname:       String,
    pub identity:       String,
    pub publication:    Tm,
    // XXX: can this ever be IPv6?
    pub ip:             Ipv4Addr,
    pub orport:         u32,
    pub dirport:        u32,

    /* 'm' line */
    pub digest:         String,

    /* 's' line */
    pub flags:          Vec<Flag>,

    /* 'v' line */
    pub version:        Option<String>,

    /* 'w' line */
    pub bandwidth:      Option<u32>,
    pub unmeasured:     Option<bool>,
}

#[derive(Debug)]
pub struct Footer;

#[cfg(test)]
mod tests {
    use directory::directory_grammar::{micro_status_entry};
    
    #[test]
    fn minimal_micro_status_entry() {
        let s = "r PDrelay1 AAFJ5u9xAqrKlpDW6N0pMhJLlKs 2015-09-11 04:57:25 95.215.44.189 8080 0\nm 3zNMfCiqAaDtvak3divNYzeYxh/f9OiCBcs6YKU1RrU\ns Fast Running Stable Valid\nv Tor 0.2.7.2-alpha-dev\nw Bandwidth=549\n";
        println!("{:?}", micro_status_entry(s));
    }
}
