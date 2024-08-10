pub trait AppendStr {
    fn append_to(&self, out: &mut String);

    fn append_len(&self) -> usize;
}

impl<T> AppendStr for &T
where
    T: AppendStr,
{
    fn append_to(&self, out: &mut String) {
        T::append_to(self, out)
    }

    fn append_len(&self) -> usize {
        T::append_len(self)
    }
}

impl AppendStr for &str {
    fn append_to(&self, out: &mut String) {
        out.push_str(self);
    }

    fn append_len(&self) -> usize {
        self.len()
    }
}

impl AppendStr for String {
    fn append_to(&self, out: &mut String) {
        out.push_str(self);
    }

    fn append_len(&self) -> usize {
        self.len()
    }
}

impl AppendStr for char {
    fn append_to(&self, out: &mut String) {
        out.push(*self)
    }

    fn append_len(&self) -> usize {
        self.len_utf8()
    }
}
