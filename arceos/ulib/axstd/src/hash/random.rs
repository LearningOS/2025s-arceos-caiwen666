use core::hash::BuildHasher;
use core::hash::Hasher;
use core::hash::SipHasher;

use crate::random::hashmap_random_keys;

#[derive(Clone, Debug)]
pub struct RandomState {
    k0: u64,
    k1: u64,
}

impl RandomState {
    #[inline]
    pub fn new() -> Self {
        let (k0, k1) = hashmap_random_keys();
        Self { k0, k1 }
    }
}

impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;

    #[inline]
    fn build_hasher(&self) -> DefaultHasher {
        DefaultHasher(SipHasher::new_with_keys(self.k0, self.k1))
    }
}

#[derive(Clone, Debug)]
pub struct DefaultHasher(SipHasher);

impl DefaultHasher {
    #[inline]
    pub fn new() -> DefaultHasher {
        DefaultHasher(SipHasher::new_with_keys(0, 0))
    }
}

impl Default for DefaultHasher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for DefaultHasher {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.write(msg)
    }

    #[inline]
    fn write_str(&mut self, s: &str) {
        self.0.write_str(s);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0.finish()
    }
}

impl Default for RandomState {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
