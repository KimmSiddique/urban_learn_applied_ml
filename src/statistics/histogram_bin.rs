pub(crate) struct HistogramBin {
    start: usize,
    end: usize,
    count: usize,
}

impl HistogramBin {
    pub(crate) fn new(start: usize, end: usize, count: usize) -> Self {
        Self { start, end, count }
    }

    pub(crate) fn get_start(&self) -> usize {
        self.start
    }
    pub(crate) fn get_end(&self) -> usize {
        self.end
    }
    pub(crate) fn get_count(&self) -> usize {
        self.count
    }
}