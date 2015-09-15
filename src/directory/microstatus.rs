use std::collections::{HashMap};
use std::net::{Ipv4Addr, Ipv6Addr};
use time::{Tm};

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct VotingDelay {
    vote_seconds:   u32,
    dist_seconds:   u32,
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
pub struct Authority;

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    pub nickname:       String,
    pub identity:       String,
    // TODO: use something other than Tm here?
    pub publication:    Tm,
    // TODO: can this ever be IPv6?
    pub ip:             Ipv4Addr,
    pub orport:         u32,
    pub dirport:        u32,
    pub digest:         String,
    pub flags:          Vec<Flag>,
    pub version:        Option<String>,
    pub bandwidth:      Option<u32>,
    pub unmeasured:     Option<bool>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Footer;

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr};
    use time::{Tm};

    use directory::directory_grammar::{micro_status_entry, micro_status_preamble};
    use super::*;
    
    #[test]
    fn minimal_micro_status_entry() {
        let s = "r PDrelay1 AAFJ5u9xAqrKlpDW6N0pMhJLlKs 2015-09-11 04:57:25 95.215.44.189 8080 0\nm 3zNMfCiqAaDtvak3divNYzeYxh/f9OiCBcs6YKU1RrU\ns Fast Running Stable Valid\nv Tor 0.2.7.2-alpha-dev\nw Bandwidth=549\n";

        let expected = Entry {
            nickname:       "PDrelay1".to_string(),
            identity:       "AAFJ5u9xAqrKlpDW6N0pMhJLlKs".to_string(),
            publication:    Tm {
                                    tm_year:    115,
                                    tm_mon:     8,
                                    tm_mday:    11,
                                    tm_hour:    4,
                                    tm_min:     57,
                                    tm_sec:     25,
                                    tm_wday:    0,
                                    tm_yday:    0,
                                    tm_isdst:   0,
                                    tm_utcoff:  0,
                                    tm_nsec:    0,
                                },
            ip:             "95.215.44.189".parse().unwrap(),
            orport:         8080,
            dirport:        0,
            digest:         "3zNMfCiqAaDtvak3divNYzeYxh/f9OiCBcs6YKU1RrU"
                                .to_string(),
            flags:          vec![Flag::Fast, Flag::Running, Flag::Stable,
                                 Flag::Valid],
            version:        Some("Tor 0.2.7.2-alpha-dev".to_string()),
            bandwidth:      Some(549),
            unmeasured:     None,
        };

        let result = micro_status_entry(s).ok().expect("parsing failed!");
        assert_eq!(result, expected);
    }

    #[test]
    fn minimal_micro_preamble() {
        let s = "network-status-version 3 microdesc\nvote-status consensus\nconsensus-method 20\nvalid-after 2015-09-11 21:00:00\nfresh-until 2015-09-11 22:00:00\nvalid-until 2015-09-12 00:00:00\nvoting-delay 300 300\nclient-versions 0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,0.2.7.1-alpha,0.2.7.2-alpha\nserver-versions 0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,0.2.7.1-alpha,0.2.7.2-alpha\nknown-flags Authority BadExit Exit Fast Guard HSDir Running Stable V2Dir Valid\nparams CircuitPriorityHalflifeMsec=30000 NumDirectoryGuards=3 NumEntryGuards=1 NumNTorsPerTAP=100 Support022HiddenServices=0 UseNTorHandshake=1 UseOptimisticData=1 bwauthpid=1 cbttestfreq=1000 pb_disablepct=0 usecreatefast=0\n";
        assert_eq!(micro_status_preamble(s), Ok(()));
    }
}
