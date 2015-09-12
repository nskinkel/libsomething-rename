
#[cfg(test)]
mod tests {
    use directory::directory_grammar::{micro_status_entry};
    
    #[test]
    fn minimal_micro_status_entry() {
        let s = "r PDrelay1 AAFJ5u9xAqrKlpDW6N0pMhJLlKs 2015-09-11 04:57:25 95.215.44.189 8080 0\nm 3zNMfCiqAaDtvak3divNYzeYxh/f9OiCBcs6YKU1RrU\ns Fast Running Stable Valid\nv Tor 0.2.7.2-alpha-dev\nw Bandwidth=549\n";
        assert_eq!(micro_status_entry(s), Ok(()));
    }
}
