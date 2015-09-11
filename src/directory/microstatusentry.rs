use std::net::{Ipv4Addr, Ipv6Addr};
use time::{Tm};

/* TODO: docs */
pub struct MicroStatusEntry {
    /* 'r' line */
    nickname:       String,   /* the OR's nickname */
    // TODO: custom type?
    identity:       XXX,      /* a hash of the OR's identity key, encoded in
                                 base64, with the trailing equals sign(s)
                                 removed */
    publication:    Tm,       /* publication time of this OR's most recent
                                 descriptor, in UTC YYYY-MM-DD HH:MM:SS */
    // TODO: can this be IPv6?
    ip:             Ipv4Addr, /* current IP address */
    orport:         u32,      /* current OR port */
    dirport:        u32,      /* current directory port, or 0 for none */

    /* 'm' line */
    // TODO: custom type?
    digest:         XXX,     /* base64 of the SHA256 hash of the router's
                                microdescriptor with trailing "="'s stripped */
}
