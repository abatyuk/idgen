use std::time::{SystemTime, UNIX_EPOCH, Duration};

struct IDGenConfig {
    machine_id_mask: u64,
    timestamp_mask: u64,
    timestamp_shift: u8,
    // Max seq number size is 64 - timestamp bits - machine id bits
    // Max seq number is 2^(max key size) - 1
    // Maximum seq number size is 64 - 41 - 1 = 22, so maximum sequence number is 4194303
    // Making it u64 to avoid conversion at comparison/BitOr
    max_seq_no: u64
}

pub struct IDGen {
    config: IDGenConfig,
    current_seq_no: u64,
    since: u64
}

impl IDGen {
    pub fn new(machine_id: u8) -> Self {
        IDGen::new_with_config(machine_id, 8, 41)
    }

    pub fn new_with_config(machine_id: u8, machine_id_bits: u8, timestamp_bits: u8) -> Self {
        let config = IDGenConfig::new(machine_id, machine_id_bits, timestamp_bits);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        IDGen {
            config,
            current_seq_no: 0,
            since: now
        }
    }

    pub fn new_id(&mut self) -> u64 {
        let mut now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        if self.since > now {
            panic!("Time went back")
        } else if self.since == now {
            self.current_seq_no = (self.current_seq_no + 1) & self.config.max_seq_no;

            let hundred_micros = Duration::new(0, 100*1000);
            if self.current_seq_no == 0 {
                while self.since == now {
                    // Sleep for hundred microseconds until timestamp changes
                    std::thread::sleep(hundred_micros);
                    now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                }
                self.since = now;
                self.current_seq_no = 0
            }
        } else {
            self.since = now;
            self.current_seq_no = 0
        }

        ((self.config.timestamp_mask & self.since) << self.config.timestamp_shift) | self.config.machine_id_mask | self.current_seq_no
    }
}

impl IDGenConfig {
    fn new(machine_id: u8, machine_id_bits: u8, timestamp_bits: u8) -> Self {
        assert!(0 < machine_id_bits && machine_id_bits <= 8);
        assert!(machine_id < ((1 << machine_id_bits) as u16 - 1) as u8) ;
        assert!(41 <= timestamp_bits && timestamp_bits <= 43);
        let max_seq_bits = 64 - timestamp_bits - machine_id_bits;
        IDGenConfig {
            machine_id_mask: (machine_id as u64) << (64 - machine_id_bits),
            timestamp_mask: ((1 as u64) << timestamp_bits) - 1,
            timestamp_shift: 64 - timestamp_bits - machine_id_bits,
            max_seq_no: ((1 as u64) << max_seq_bits) - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IDGen;
    use std::collections::{HashSet};

    #[test]
    fn config() {
        let mut idgen = IDGen::new(0);

        let range:Vec<u64> = (0..8000000).map(|_i| idgen.new_id()).collect();
        let mut uniq = HashSet::new();
        let all_ids_unique = range.into_iter().all(|id| uniq.insert(id));
        assert!(all_ids_unique);
    }
}
