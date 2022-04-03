#[derive(Debug)]
pub struct OrderMap<'a, V> {
    keys: Vec<&'a str>,
    values: Vec<V>,
}

impl<'a, V> OrderMap<'a, V> {
    pub fn new() -> Self {
        return OrderMap {
            keys: Vec::new(),
            values: Vec::new(),
        };
    }

    pub fn get_keys(&self) -> &Vec<&str> {
        return &self.keys;
    }

    pub fn get_values(&self) -> &Vec<V> {
        return &self.values;
    }

    pub fn len(&self) -> usize {
        return self.keys.len();
    }

    pub fn get_index(&self, key: &str) -> (usize, bool) {
        let mut right = self.len();
        if right == 0 {
            return (0, false);
        }
        let mut left: usize = 0;
        let mut index = right / 2;
        while left < right {
            let p_key = match self.keys.get(index) {
                Some(&val) => val,
                None => return (0, false),
            };
            if p_key == key {
                return (index, true);
            } else if p_key < key {
                left = index + 1
            } else if p_key > key {
                right = index
            }
            index = (left + right) / 2
        }
        return (index, false);
    }

    pub fn get(&self, k: &str) -> Option<&V> {
        let (index, ok) = self.get_index(k);
        if ok {
            return Some(self.values.get(index).unwrap());
        } else {
            return None;
        }
    }

    pub fn get_by_index(&self, index: usize) -> Option<(&str, &V)> {
        return match self.keys.get(index) {
            Some(&s) => Some((s, self.values.get(index).unwrap())),
            None => None,
        };
    }

    pub fn get_key_by_index(&self, index: usize) -> Option<&str> {
        return match self.keys.get(index) {
            Some(&s) => Some(s),
            None => None,
        };
    }

    pub fn get_value_by_index(&self, index: usize) -> Option<&V> {
        return self.values.get(index);
    }

    fn insert_at(&mut self, k: &'a str, v: V, index: usize) {
        self.keys.insert(index, k);
        self.values.insert(index, v);
    }

    pub fn insert(&mut self, k: &'a str, v: V) -> bool {
        let (index, ok) = self.get_index(k);
        if ok {
            return false;
        } else {
            self.keys.insert(index, k);
            self.values.insert(index, v);
            return true;
        }
    }

    pub fn set(&mut self, k: &'a str, v: V) {
        let (index, ok) = self.get_index(k);
        if ok {
            self.keys[index] = k;
            self.values[index] = v;
        } else {
            self.keys.insert(index, k);
            self.values.insert(index, v);
        }
    }

    pub fn delete(&mut self, k: &'a str, v: V) -> bool {
        let (index, ok) = self.get_index(k);
        if ok {
            self.keys.remove(index);
            self.values.remove(index);
            return true;
        } else {
            return false;
        }
    }

    pub fn delete_by_index(&mut self, index: usize) -> bool {
        if index >= self.keys.len() {
            return false;
        } else {
            self.keys.remove(index);
            self.values.remove(index);
            return true;
        }
    }
}
