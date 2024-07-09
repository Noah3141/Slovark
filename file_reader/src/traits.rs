///
/// A type implements searchable if the type can call .find(subelement) and return back the index for the subelement within the type
pub trait Searchable {
    fn find(self, item: Self) -> Option<usize>;
    fn find_all(self, item: Self) -> Vec<usize>;
}

impl Searchable for &[u8] {
    fn find(self, item: Self) -> Option<usize> {
        self.windows(item.len()).position(|where_window| where_window == item)
    }

    fn find_all(self, item: Self) -> Vec<usize> {
        let mut idxs = vec![];
        let mut start_idx: usize = 0;
        while let Some(idx) = self[start_idx..].find(item) {
            let global_idx = start_idx + idx;
            idxs.push(global_idx);
            start_idx = global_idx + item.len();
        }
        return idxs
    }
}