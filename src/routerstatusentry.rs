use std::net::{Ipv4Addr, Ipv6Addr};
use time::{Tm};

#[derive(Debug, Eq, PartialEq)]
pub enum Flag {
    Authority,  /* the router is a directory authority */
    BadExit,    /* the router is believed to be useless as an exit node */
    Exit,       /* the router is more useful for building general-purpose exit
                   circuits than for relay circuits */
    Fast,       /* the router is suitable for high-bandwidth circuits */
    Guard,      /* the router is suitable for use as an entry guard */
    HSDir,      /* the router is considered a v2 hidden service directory */
    Named,      /* the router's identity-nickname mapping is canonical */
    Stable,     /* the router is suitable for long-lived circuits */
    Running,    /* the router is currently usable */
    Unnamed,    /* another router has bound the name used by this router */
    Valid,      /* the router has been 'validated' */
    V2Dir,      /* the router implements the v2 directory protocol or higher */
}

pub enum ORAddressImpl {
    ipv4 {address: Ipv4Addr},
    ipv6 {address: Ipv6Addr},
}

pub struct ORAddress {
    port: u32,
    address: ORAddressImpl,
}

/* TODO: docs */
pub struct RouterStatusEntry {
    /* 'r' line */
    nickname:       String,   /* the OR's nickname */
    // TODO: custom type?
    identity:       XXX,      /* a hash of the OR's identity key, encoded in
                                 base64, with the trailing equals sign(s)
                                 removed */
    // TODO: custom type?
    digest:         XXX,      /* a hash of the OR's most recent descriptor (not
                                 including the signature) in base64 */
    publication:    Tm,       /* publication time of this OR's most recent
                                 descriptor, in UTC YYYY-MM-DD HH:MM:SS */
    // TODO: can this be IPv6?
    ip:             Ipv4Addr, /* current IP address */
    orport:         u32,      /* current OR port */
    dirport:        u32,      /* current directory port, or 0 for none */

    /* 'a' line */
    oraddresses:    Vec<ORAddress>, /* TODO: docs */    

    /* 's' line */
    flags:          Vec<Flags>,     /* TODO: docs */

    /* 'v' line */
    version:        String,   /* Tor protocol version this relay is running */

    /* 'w' line */
    bandwidth:      u32,      /* TODO: docs */
    measured:       u32,      /* TODO: docs */
    unmeasured:     u32,      /* TODO: docs */


}
