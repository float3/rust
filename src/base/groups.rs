use crate::defaults::StringType;

pub(crate) struct Groups {
    list: Vec<StringType>,
}

impl Groups {
    // pub(crate) fn _valid_name(&self, value: String) {
    //     todo!()
    // }

    pub(crate) fn append(&self, value: String) {
        // self._valid_name(value);
        if !self.list.contains(&value) {
            self.list.push(value);
        }
    }

    pub(crate) fn __setitem__(&self, i: usize, value: String) {
        // self._valid_name(value);
        self.list = value;
    }

    pub(crate) fn __eq__(&self, other: Groups) {
        if self.list.len() != other.list.len() {
            return false;
        }

        for (i, j) in self.list.iter().zip(other.list.iter()) {
            if i.to_lowercase() != j.to_lowercase() {
                return false;
            }
        }

        return true;
    }
}
