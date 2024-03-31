use rand::seq::SliceRandom;
use rand::RngCore;
use sha256::{digest, try_digest};
use crate::config;
use lrwn::DevAddr;
use std::path::Path;                                                                                
use std::fs;                                                                                        
use std::fs::File;                                                                                  
use std::io::Read;                                                                                  
use std::io::Write;
use bincode;

const NUMBER_OF_SLOTS: u32 = 3;
static SLOTS_TAKEN: u32 = 0;
const FILENAME: &str = "devaddr_slots.json";                                                        
static SLOTS: [bool; NUMBER_OF_SLOTS] = [false, false, false];

pub fn get_random_dev_addr() -> DevAddr {
    let conf = config::get();
    let mut rng = rand::thread_rng();

    // Get configured DevAddr prefixes.
    let prefixes = if conf.network.dev_addr_prefixes.is_empty() {
        vec![conf.network.net_id.dev_addr_prefix()]
    } else {
        conf.network.dev_addr_prefixes.clone()
    };

    // Pick a random one (in case multiple prefixes are configured).
    let prefix = *prefixes.choose(&mut rng).unwrap();

    // Generate random DevAddr.
    let mut dev_addr: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut dev_addr);
    #[cfg(test)]
    {
        dev_addr = [1, 2, 3, 4];
    }
    let mut dev_addr = DevAddr::from_be_bytes(dev_addr);

    // Set DevAddr prefix.
    dev_addr.set_dev_addr_prefix(prefix);

    // find the dev_addr slot
    let sha256val = digest(bincode::serialize(&dev_addr).unwrap());
    let slot_value: u32 = sha256val.chars().map(|x| x as u32).sum::<u32>() % NUMBER_OF_SLOTS;

    // check whether slots are left.
    // panic if none left.
    if SLOTS_TAKEN == NUMBER_OF_SLOTS {
        panic!("Program run our of slot values! All of the {} slots are used up", NUMBER_OF_SLOTS);
    }

    // check whether a given slot value is already taken
    match SLOTS[slot_value] {
        true => {
            // slot is already taken, generate again
            return get_random_dev_addr();
        },
        false => {
            // mark the slots as taken
            SLOTS[slots_value] = true;
        },
    };


    // increase the number of taken slots
    SLOTS_TAKEN = SLOTS_TAKEN + 1;

    dev_addr
}
