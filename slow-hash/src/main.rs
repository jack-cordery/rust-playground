use std::hash::{DefaultHasher, Hash, Hasher};

/// here we are going to implement a really bad hashmap
/// we are going to implement the following
/// a contiguous heap allocated data struct to store the (K,V) pairs
/// - luckily Vec is contiguous so we can use that as the underlying data struct
/// - after we may want to implement our own
/// - which would look something like ptr and capacity (which would be size_of (K) size_of (val))
/// - and then offset at those values
/// - insert which will hash and then mod to get an index
/// - on collision we will do open addressing (that is to keep going until we find an open slot)
/// - on retrieval we will do so in reverse
/// - we will use std::hash (or whatever it is)
/// - we will seek O(N) retrieval and insertion and mem
/// TODO: - deal with collisions by implementing open addressing
#[derive(Debug, Clone)]
struct KeyVal {
    key: String,
    value: u8,
}

#[derive(Debug)]
struct HashMap {
    vec: Vec<Option<KeyVal>>,
}

impl HashMap {
    fn new(capacity: usize) -> Self {
        Self {
            vec: vec![None; capacity],
        }
    }

    fn insert(&mut self, kv: KeyVal) {
        let mut hasher = DefaultHasher::new();
        kv.key.hash(&mut hasher);
        let k_hash = hasher.finish();
        let index = k_hash % self.vec.len() as u64;
        self.vec[index as usize] = Some(kv);
    }

    fn get(&self, key: &str) -> Option<&KeyVal> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let k_hash = hasher.finish();
        let index = k_hash % self.vec.len() as u64;

        self.vec[index as usize].as_ref()
    }
}

fn main() {
    let mut h = HashMap::new(10);

    let kv = KeyVal {
        key: String::from("hello"),
        value: 1,
    };
    let kv2 = KeyVal {
        key: String::from("world"),
        value: 2,
    };

    h.insert(kv);
    h.insert(kv2);

    let g = h.get("hello");
    let v = g.unwrap().value;
    println!("g:{g:?} v: {v:?}");
    println!("h: {h:?}");
}
