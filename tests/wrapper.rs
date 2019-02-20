extern crate web_gettext;


#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::prelude::*;
    use web_gettext::wrapper::WrappedTranslator;

    #[test]
    pub fn wrong_mo() {
        if let Some(_) = WrappedTranslator::new(b"xxx") {
            assert!(false)  // Should return None
        }
    }

    #[test]
    pub fn integration() {
        let mut f = File::open("test_data/integration.mo").unwrap();
        let mut data: Vec<u8> = vec![];
        f.read_to_end(&mut data).unwrap();
        let translator = WrappedTranslator::new(&data[..]).unwrap();

        // not found
        assert_eq!(translator.gettext(String::from("not translated")), "not translated");

        // existing
        assert_eq!(translator.gettext(String::from("existent")), "egzistuojantis");

        // plurals
        assert_eq!(
            translator.ngettext(String::from("a good string"), String::from("good strings"), 1),
            "gera eilute",
        );
        assert_eq!(
            translator.ngettext(String::from("a good string"), String::from("good strings"), 2),
            "geros eilutes"
        );
    }
}
