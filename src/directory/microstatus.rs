// TODO: remove these
#![allow(dead_code)]
#![allow(unused_imports)]

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
pub struct Preamble {
    pub method:             u32,
    pub valid_after:        Tm,
    pub fresh_until:        Tm,
    pub valid_until:        Tm,
    // TODO: make this an enum or something
    pub voting_delay:       (u32, u32),
    pub client_versions:    Option<String>,
    pub server_versions:    Option<String>,
    // XXX use vec of enums instead?
    pub known_flags:        Vec<String>,
    pub params:             Option<HashMap<String, i32>>,
}

// TODO: move this elsewhere
#[derive(Debug, Eq, PartialEq)]
pub struct DirectoryAuthority {
    pub nickname:   String,
    pub identity:   String,
    pub address:    String,
    // TODO: can these be Ipv6?
    pub ip:         Ipv4Addr,
    pub dirport:    u16,
    pub orport:     u16,
    pub contact:    String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct DirSource {
    pub authority:      DirectoryAuthority,
    pub vote_digest:    String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Authority {
    authorities: Vec<DirSource>,
}

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
    // TODO: what type should this actually be? (almost certainly not
    //       Option<bool>)
    pub unmeasured:     Option<bool>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Footer;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use std::net::{Ipv4Addr};
    use time::{Tm};

    use directory::directory_grammar::{micro_status_entry, micro_status_preamble, micro_status_authority};
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
    fn minimal_micro_status_preamble() {
        let s = "network-status-version 3 microdesc\nvote-status consensus\nconsensus-method 20\nvalid-after 2015-09-11 21:00:00\nfresh-until 2015-09-11 22:00:00\nvalid-until 2015-09-12 00:00:00\nvoting-delay 300 300\nclient-versions 0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,0.2.7.1-alpha,0.2.7.2-alpha\nserver-versions 0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,0.2.7.1-alpha,0.2.7.2-alpha\nknown-flags Authority BadExit Exit Fast Guard HSDir Running Stable V2Dir Valid\nparams CircuitPriorityHalflifeMsec=30000 NumDirectoryGuards=3 NumEntryGuards=1 NumNTorsPerTAP=100 Support022HiddenServices=0 UseNTorHandshake=1 UseOptimisticData=1 bwauthpid=1 cbttestfreq=1000 pb_disablepct=0 usecreatefast=0\n";

        let mut h: HashMap<String, i32> = HashMap::new();
        h.insert("NumEntryGuards".to_string(), 1);
        h.insert("bwauthpid".to_string(), 1);
        h.insert("NumNTorsPerTAP".to_string(), 100);
        h.insert("usecreatefast".to_string(), 0);
        h.insert("cbttestfreq".to_string(), 1000);
        h.insert("UseOptimisticData".to_string(), 1);
        h.insert("Support022HiddenServices".to_string(), 0);
        h.insert("UseNTorHandshake".to_string(), 1);
        h.insert("CircuitPriorityHalflifeMsec".to_string(), 30000);
        h.insert("pb_disablepct".to_string(), 0);
        h.insert("NumDirectoryGuards".to_string(), 3);

        let expected = Preamble {
            method: 20,
            valid_after: Tm {
                                tm_sec: 0,
                                tm_min: 0,
                                tm_hour: 21,
                                tm_mday: 11,
                                tm_mon: 8,
                                tm_year: 115,
                                tm_wday: 0,
                                tm_yday: 0,
                                tm_isdst: 0,
                                tm_utcoff: 0,
                                tm_nsec: 0,
                            },
            fresh_until: Tm {
                                tm_sec: 0,
                                tm_min: 0,
                                tm_hour: 22,
                                tm_mday: 11,
                                tm_mon: 8,
                                tm_year: 115,
                                tm_wday: 0,
                                tm_yday: 0,
                                tm_isdst: 0,
                                tm_utcoff: 0,
                                tm_nsec: 0,
                            },
            valid_until: Tm { 
                                tm_sec: 0,
                                tm_min: 0,
                                tm_hour: 0,
                                tm_mday: 12,
                                tm_mon: 8,
                                tm_year: 115,
                                tm_wday: 0,
                                tm_yday: 0,
                                tm_isdst: 0,
                                tm_utcoff: 0,
                                tm_nsec: 0,
                            },
            voting_delay: (300, 300),
            client_versions: Some("0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,\
                                  0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,\
                                  0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,\
                                  0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,\
                                  0.2.7.1-alpha,0.2.7.2-alpha".to_string()),
            server_versions: Some("0.2.4.23,0.2.4.24,0.2.4.25,0.2.4.26,\
                                  0.2.4.27,0.2.5.8-rc,0.2.5.9-rc,0.2.5.10,\
                                  0.2.5.11,0.2.5.12,0.2.6.5-rc,0.2.6.6,\
                                  0.2.6.7,0.2.6.8,0.2.6.9,0.2.6.10,\
                                  0.2.7.1-alpha,0.2.7.2-alpha".to_string()),
            known_flags: vec!["Authority".to_string(),
                              "BadExit".to_string(),
                              "Exit".to_string(),
                              "Fast".to_string(),
                              "Guard".to_string(),
                              "HSDir".to_string(),
                              "Running".to_string(),
                              "Stable".to_string(),
                              "V2Dir".to_string(),
                              "Valid".to_string()],
            params: Some(h),
        };
        let result = micro_status_preamble(s).ok().expect("failed!");
        assert_eq!(result, expected);
    }

    #[test]
    fn minimal_micro_status_authority() {
        let s = "dir-source longclaw 23D15D965BC35114467363C165C4F724B64B4F66 longclaw.riseup.net 199.254.238.52 80 443\ncontact Riseup Networks <collective at riseup dot net> - 1nNzekuHGGzBYRzyjfjFEfeisNvxkn4RT\nvote-digest A04CD308EE61BAC3B40232F12001E167DC5903BF\n";
        let d = DirectoryAuthority {
            nickname:   "longclaw".to_string(),
            identity:   "23D15D965BC35114467363C165C4F724B64B4F66".to_string(),
            address:    "longclaw.riseup.net".to_string(),
            ip:         "199.254.238.52".parse().unwrap(),
            dirport:    80,
            orport:     443,
            contact:    "Riseup Networks <collective at riseup dot net> - 1nNzekuHGGzBYRzyjfjFEfeisNvxkn4RT".to_string(),
        };

        let expected = DirSource {
            authority: d,
            vote_digest: "A04CD308EE61BAC3B40232F12001E167DC5903BF".to_string(),
        };

        let result = micro_status_authority(s).ok().expect("failed!");
        assert_eq!(result, expected);
    }
}
