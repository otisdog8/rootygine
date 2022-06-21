//! Evict carefully, but using a big table prevents problems. Storing
//! upperbound and lowerbound and other crap is also important. This should probably include 
//! refutations in the future and other optimizations.

use log::debug;
use std::{
    collections::HashMap,
    // ops::{Deref, DerefMut},
};

use chess::Board;

// TODO: use size in memory instead of absolute size
const MAX_ENTRIES: usize = 100000000;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TransTableEntry {
    pub flag: Flag,
    pub eval: f32,
    pub depth: i32,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Flag {
    Lowerbound,
    Upperbound,
    Exact,
    None,
}

pub struct TransTable {
    /// Wrapper of transposition table so it associates hash with an entry
    // Do NOT directly write
    pub tt: HashMap<u64, TransTableEntry>,
}


#[allow(dead_code)]
impl TransTable {
    /// add_entry needs to be by itself because it needs to check if it needs
    /// to evict and or to insert.

    pub fn new() -> Self {
        // Allocate all the memory at once because it's expensive af to do on the fly
        TransTable {
            tt: HashMap::with_capacity(MAX_ENTRIES),
        }
    }

    pub fn add_entry(&mut self, board: Board, entry: TransTableEntry) {
        // This takes care of the size of the table
        if need_to_evict(&self.tt) {
            evict(&mut self.tt);
            // Evicted from trans table
        }

        self.tt.insert(board.get_hash(), entry);
    }
}

fn need_to_evict(table: &HashMap<u64, TransTableEntry>) -> bool {
    // Simple function, exists for readability
    table.len() > MAX_ENTRIES
}

fn evict(table: &mut HashMap<u64, TransTableEntry>) {
    // Currently just evicts down to MAX_ENTRIES - 20% with lowest depth if too full.
    debug!(
        "Current size of transposition table: {} entries",
        table.len()
    );
    let num_to_evict = table.len() - (MAX_ENTRIES - (MAX_ENTRIES / 5));
    let mut num_evicted = 0;
    let mut eviction_list: Vec<u64> = vec![];

    debug!(
        "Started attempting to evict {} entries from transposition table",
        num_to_evict
    );

    debug!("Getting eviction list");
    while num_evicted < num_to_evict {
        least_depth_entries(table, &mut eviction_list);

        while !eviction_list.is_empty() && num_evicted < num_to_evict {
            table.remove(&eviction_list.pop().expect("This should not be possible"));
            num_evicted += 1;
        }
    }

    debug!("Evicted {} entries from transposition tables", num_to_evict);
}

fn least_depth_entries(table: &HashMap<u64, TransTableEntry>, eviction_list: &mut Vec<u64>) {
    let mut lowest_depth = i32::MAX;

    // Find the lowest depth in the map
    for trans_table_entry in table.values() {
        if trans_table_entry.depth < lowest_depth {
            lowest_depth = trans_table_entry.depth;
        }
    }

    debug!("Found lowest depth value of {}", lowest_depth);

    // Returns all of keys of the lowest depth
    for (board, trans_table_entry) in table {
        if trans_table_entry.depth == lowest_depth {
            eviction_list.push(*board);
        }
    }

    debug!("Got {} keys of lowest depth.", eviction_list.len());
}
