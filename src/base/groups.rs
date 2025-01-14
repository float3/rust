use crate::defaults::StringType;

pub(crate) struct Groups {
    list: Vec<StringType>,
}

impl Groups {
    // pub(crate) fn _valid_name(&self, value: String) {
    //     todo!()
    // }

    pub(crate) fn append(&mut self, value: String) {
        // self._valid_name(value);
        if !self.list.contains(&value) {
            self.list.push(value);
        }
    }

    pub(crate) fn __setitem__(&mut self, i: usize, value: String) {
        // self._valid_name(value);
        self.list[i] = value;
    }
}

impl PartialEq for Groups {
    fn eq(&self, other: &Self) -> bool {
        if self.list.len() != other.list.len() {
            return false;
        }

        for (i, j) in self.list.iter().zip(other.list.iter()) {
            if i.to_lowercase() != j.to_lowercase() {
                return false;
            }
        }

        true
    }
}

impl Eq for Groups {}
