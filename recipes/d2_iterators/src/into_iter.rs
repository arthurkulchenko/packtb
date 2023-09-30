#[derive(Clone)]
pub struct Company {
    ceo: String,
    receptionist: String,
    marketing: String,
}

pub struct CompanyIter<'a> {
    company: &'a Company,
    index: usize,
}

impl<'a> Iterator for CompanyIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let match_result = match self.index {
            1 => Some(self.company.ceo.as_str()),
            2 => Some(self.company.receptionist.as_str()),
            3 => Some(self.company.marketing.as_str()),
            _ => None,
        };
        self.index += 1;
        match_result
    }
}

impl<'a> IntoIterator for &'a Company {
    type IntoIter = CompanyIter<'a>;
    type Item = &'a str;
    fn into_iter(self) -> Self::IntoIter {
        CompanyIter { company: self, index: 1, }
    }
}

#[cfg(test)]
    mod test_company {
    use crate::into_iter::Company;

    #[test]
    fn test_intoiter() {
        let company = Company {
            ceo: "Enoe Done".to_string(),
            receptionist: "John Dsu".to_string(),
            marketing: "Jane Molk".to_string(),
        };
        let binding = company.clone();
        let mut iter = binding.into_iter();
        assert_eq!(iter.next(), Some("Enoe Done"));
        assert_eq!(iter.next(), Some("John Dsu"));
        assert_eq!(iter.next(), Some("Jane Molk"));
        let mut result = String::new();
        for name in &company {
            result.push_str(name);
        }
        assert_eq!(result, "Enoe DoneJohn DsuJane Molk");
    }
}
