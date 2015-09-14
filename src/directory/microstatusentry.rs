use std::net::{Ipv4Addr, Ipv6Addr};
use time::{Tm};

/* TODO: docs */
#[derive(Debug)]
pub struct MicroStatusEntry {
    /* 'r' line */
    pub nickname:       String,   /* the OR's nickname */
    // TODO: custom type?
    pub identity:       String,      /* a hash of the OR's identity key, encoded in
                                 base64, with the trailing equals sign(s)
                                 removed */
    //publication:    Tm,       /* publication time of this OR's most recent
    //                             descriptor, in UTC YYYY-MM-DD HH:MM:SS */
    pub publication:      String,

    // TODO: can this be IPv6?
    //ip:             Ipv4Addr, /* current IP address */
    pub ip:             String,
    pub orport:         u32,      /* current OR port */
    pub dirport:        u32,      /* current directory port, or 0 for none */

    /* 'm' line */
    // TODO: custom type?
    pub digest:         String,     /* base64 of the SHA256 hash of the router's
                                microdescriptor with trailing "="'s stripped */
}
#[cfg(test)]
mod tests {
    use directory::directory_grammar::{micro_status_entry};
    
    #[test]
    fn minimal_micro_status_entry() {
        let s = "r PDrelay1 AAFJ5u9xAqrKlpDW6N0pMhJLlKs 2015-09-11 04:57:25 95.215.44.189 8080 0\nm 3zNMfCiqAaDtvak3divNYzeYxh/f9OiCBcs6YKU1RrU\ns Fast Running Stable Valid\nv Tor 0.2.7.2-alpha-dev\nw Bandwidth=549\n";
        //assert_eq!(micro_status_entry(s), Ok(()));
        println!("{:?}", micro_status_entry(s));
    }
}
