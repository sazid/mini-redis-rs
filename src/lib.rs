use std::{collections::HashMap, sync::Mutex};

pub fn hash(s: String) -> usize {
    const P: usize = 31;
    const MOD: usize = 1e9 as usize + 7;

    let mut pow = vec![1usize; s.len()];
    for i in 1..s.len() {
        pow[i] = (pow[i - 1] * P) % MOD;
    }

    let mut hash = 0;
    for (i, &c) in s.as_bytes().iter().enumerate() {
        hash = (hash + ((c as usize * pow[i]) % MOD)) % MOD;
    }
    hash
}

pub struct ShardDb<K: ToString, V> {
    shards: Vec<Mutex<HashMap<K, V>>>,
}

impl<K: ToString, V> ShardDb<K, V> {
    pub fn new(size: usize) -> ShardDb<K, V> {
        let mut shards = vec![];
        for _ in 0..size {
            shards.push(Mutex::new(HashMap::new()));
        }

        ShardDb { shards }
    }

    pub fn get(&self, key: K) -> &Mutex<HashMap<K, V>> {
        &self.shards[hash(key.to_string()) % self.shards.len()]
    }
}
