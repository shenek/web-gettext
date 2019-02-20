use gettext::Catalog;
use std::cmp::min;

struct ReaderWrapper {
    data: Vec<u8>,
    idx: usize,
}

impl ReaderWrapper {
    fn new(data: &[u8]) -> Self {
        ReaderWrapper {
            data: data.to_vec(),
            idx: 0,
        }
    }
}

impl std::io::Read for ReaderWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.idx >= self.data.len() {
            return Ok(0);
        }
        let slice = &self.data[self.idx..min(self.idx + buf.len(), self.data.len())];
        buf[..slice.len()].clone_from_slice(&slice);
        self.idx += slice.len();
        Ok(slice.len())
    }
}

#[derive(Debug)]
pub struct WrappedTranslator {
    catalog: Catalog,
}

impl WrappedTranslator {
    pub fn new(data: &[u8]) -> Option<WrappedTranslator> {
        let wrapper = ReaderWrapper::new(data);
        if let Ok(c) = Catalog::parse(wrapper) {
            Some(WrappedTranslator { catalog: c })
        } else {
            None
        }
    }

    pub fn gettext(&self, text: String) -> String {
        String::from(self.catalog.gettext(&text[..]))
    }

    pub fn ngettext(&self, singular: String, plural: String, count: u64) -> String {
        String::from(self.catalog.ngettext(&singular[..], &plural[..], count))
    }

}
