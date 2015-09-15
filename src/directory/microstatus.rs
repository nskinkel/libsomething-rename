use std::collections::{HashMap};
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
pub struct VotingDelay {
    vote_seconds:   u32,
    dist_seconds:   u32,
}

#[derive(Debug)]
pub struct Preamble {
    version:            u32,
    method:             u32,
    valid_after:        Tm,
    fresh_until:        Tm,
    valid_until:        Tm,
    voting_delay:       VotingDelay,
    client_versions:    Option<String>,
    server_versions:    Option<String>,
    // XXX use vec of enums instead?
    known_flags:        Vec<String>,
    params:             Option<HashMap<String, u32>>,
}

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
    use directory::directory_grammar::{micro_status_entry, micro_status_preamble};
    
    #[test]
    fn minimal_micro_status_entry() {
        let s = "r PDrelay1 AAFJ5u9xAqrKlpDW6N0pMhJLlKs 2015-09-11 04:57:25 95.215.44.189 8080 0\nm 3zNMfCiqAaDtvak3divNYzeYxh/f9OiCBcs6YKU1RrU\ns Fast Running Stable Valid\nv Tor 0.2.7.2-alpha-dev\nw Bandwidth=549\n";
        println!("{:?}", micro_status_entry(s));
    }

    #[test]
    fn minimal_micro_preamble() {
        let s = "network-status-version 3 microdesc\nvote-status consensus\nconsensus-method 20\nvalid-after 2015-09-11 21:00:00\nfresh-until 2015-09-11 22:00:00\nvalid-until 2015-09-12 00:00:00\nvoting-delay 300 300\nclient-versions 0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,0.2.7.1-alpha,0.2.7.2-alpha\nserver-versions 0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,0.2.7.1-alpha,0.2.7.2-alpha\nknown-flags Authority BadExit Exit Fast Guard HSDir Running Stable V2Dir Valid\nparams CircuitPriorityHalflifeMsec=30000 NumDirectoryGuards=3 NumEntryGuards=1 NumNTorsPerTAP=100 Support022HiddenServices=0 UseNTorHandshake=1 UseOptimisticData=1 bwauthpid=1 cbttestfreq=1000 pb_disablepct=0 usecreatefast=0\n";
        assert_eq!(micro_status_preamble(s), Ok(()));
    }
}
