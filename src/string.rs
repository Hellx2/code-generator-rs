pub trait StringExt {
    fn surround<T: ToString>(&self, surrounder: T) -> String
    where
        Self: Clone;

    fn surround_section<T: ToString>(&self, surrounder: T, start: usize, end: usize) -> String;
}

impl StringExt for String {
    fn surround<T: ToString>(&self, surrounder: T) -> String
    where
        Self: Clone,
    {
        let mut y = surrounder.to_string().clone();
        y.push_str(&self.clone());
        y.push_str(&surrounder.to_string());
        y
    }

    fn surround_section<T: ToString>(&self, surrounder: T, start: usize, end: usize) -> String {
        let y = self.split_at(start);
        let mut return_value = String::from(y.0);

        return_value.push_str(&surrounder.to_string());
        return_value.push_str(y.1.split_at(end).0);
        return_value.push_str(&surrounder.to_string());
        return_value.push_str(y.1.split_at(end).1);

        return_value
    }
}

impl StringExt for &str {
    fn surround<T: ToString>(&self, surrounder: T) -> String
    where
        Self: Clone,
    {
        self.to_string().surround(surrounder)
    }

    fn surround_section<T: ToString>(&self, surrounder: T, start: usize, end: usize) -> String {
        self.to_string().surround_section(surrounder, start, end)
    }
}
