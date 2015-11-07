use super::prelude::v1::*;
use super::hash::*;
use super::mem;
use super::fmt::{Debug, Display};
use core::iter;
use core::slice;

/// Number of buckets in the hash table
pub const BUCKETS: usize = 256;

/// An linked list (used for entries)
#[derive(Clone)]
pub enum LinkedList<T: Clone> {
    Elem(T, Box<LinkedList<T>>),
    Nil,
}

impl<T: Clone> LinkedList<T> {
    /// Follow
    pub fn follow(&self) -> &Self {
        use self::LinkedList::*;
        match *self {
            Elem(_, box ref l) => {
                l
            },
            Nil => {
                self
            },
        }
    }

    /// Follow mutable
    pub fn follow_mut(&mut self) -> &mut Self {
        use self::LinkedList::*;
        match *self {
            Elem(_, box ref mut l) => {
                l
            },
            Nil => {
                self
            },
        }
    }

    /// Push (consumes the list)
    pub fn push(self, elem: T) -> Self {
        LinkedList::Elem(elem, Box::new(self))
    }

    /// Iter
    pub fn iter<'a>(&'a self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            ll: self,
        }
    }
    /// Iter mut
    pub fn iter_mut<'a>(&'a mut self) -> LinkedListIterMut<'a, T> {
        LinkedListIterMut {
            ll: self,
        }
    }
}

pub struct LinkedListIter<'a, T: Clone + 'a> {
    ll: &'a LinkedList<T>,
}

impl<'a, T: Clone + 'a> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if let &LinkedList::Elem(ref e, _) = self.ll {
            Some(e)
        } else {
            None
        };
        self.ll = self.ll.follow();
        res
    }
}
pub struct LinkedListIterMut<'a, T: Clone + 'a> {
    ll: &'a mut LinkedList<T>,
}

impl<'a, T: Clone + 'a> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if let &mut LinkedList::Elem(ref mut e, _) = self.ll {
            Some(e)
        } else {
            None
        };
        self.ll = self.ll.follow_mut();
        res
    }
}

/// A entry in the hash map
#[derive(Clone)]
pub struct Entry<K: Clone, V: Clone> {
    data: LinkedList<(K, V)>,
}

impl<K: PartialEq<K> + Clone + Debug + Display, V: Clone> Entry<K, V> {
    /// Create new
    pub fn new(key: K, value: V) -> Self {
        Entry {
            data: LinkedList::Elem((key, value), Box::new(LinkedList::Nil)),
        }
    }

    /// Iter
    pub fn iter(&self) -> LinkedListIter<(K, V)> {
        self.data.iter()
    }
    /// Iter mut
    pub fn iter_mut(&self) -> LinkedListIterMut<(K, V)> {
        self.data.iter_mut()
    }

    /// Get value from entry
    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(&(_, ref v)) = self.data.iter().find(|&&(k, _)| &k == key) {
            Some(v)
        } else {
            None
        }
    }

    /// Get value mutable from entry
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {

        if let Some(&mut (_, ref mut v)) = self.data.iter_mut().find(|&&mut (k, _)| &k == key) {
            Some(v)
        } else {
            None
        }
    }

    /// Push to the list (consumes the entry returning the new one)
    pub fn push(self, key: K, value: V) -> Self {
        Entry {
            data: self.data.push((key, value)),
        }
    }
}

/// A hashtable
pub struct HashMap<K: Clone, V: Clone> {
    data: [Entry<K, V>; BUCKETS],
}

impl<K: Hash + PartialEq + Clone + Debug + Display, V: Clone + Debug> HashMap<K, V> {
    /// Make new HT
    pub fn new() -> Self {
        HashMap {
            // Sorry, but LinkedList is not, and will not be copyable
            data: [Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil} , Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}, Entry {data: LinkedList::Nil}],
        }
    }

//    /// Returns an iterator yielding the (key, value) tuples of hash map.
//    pub fn iter<'a>(&self) -> _ {
//        self.data.iter().flat_map(|x| x.iter())
//    }

    /// Get entry num
    fn get_entry(key: &K) -> usize {
        let mut s = Djb2::new();
        key.hash(&mut s);
        let res = (s.finish() % BUCKETS as u64) as usize;
        debugln!("Res is: {}", res);
        res
    }

    /// Get value
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data[Self::get_entry(key)].get(key)
    }

    /// Get value mutable
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data[Self::get_entry(key)].get_mut(key)
    }

    /// Set value (return the previous one if overwritten)
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        match self.get_mut(&key) {
            Some(r) => {
                debugln!("Key inserted: {:?}", key);
                debugln!("Old val: {:?}", r);
                debugln!("New val: {:?}", val);
                return Some(mem::replace(r, val));
            },
            _ => {}
        }
        let n = Self::get_entry(&key);
        self.data[n] = self.data[n].clone().push(key, val);
        None
    }
}

/// DJB2 hashing
pub struct Djb2 {
    state: u64,
}

impl Djb2 {
    /// Create new DJB2 hasher
    pub fn new() -> Self {
        Djb2 {
            state: 5381,
        }
    }
}

impl Hasher for Djb2 {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state = ((self.state << 5) + self.state) + b as u64;
        }
    }
}

pub fn test() {
    let mut ht = HashMap::new();

    assert!(!ht.insert(1, 42).is_some());
    assert_eq!(ht.get(&1), Some(&42));
    assert!(ht.insert(288, 666).is_some());
    assert_eq!(ht.get(&288), Some(&666));
    assert_eq!(ht.get(&1), Some(&42));


//    for i in 1..300 {
//        debugln!("Set {}", i);
//        ht.insert(i, i);
//    }
//
//    for i in 1..300 {
//        debugln!("Get {}", i);
//        assert_eq!(ht.get(&i), Some(&i));
//    }

}
