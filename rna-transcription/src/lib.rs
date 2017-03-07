#[derive(Debug, Eq, PartialEq)]
pub struct RibonucleicAcid {
    data: String,
}

impl RibonucleicAcid {
    pub fn new(data: &str) -> Self {
        RibonucleicAcid { data: data.to_string() }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct DeoxyribonucleicAcid {
    data: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(data: &str) -> Self {
        DeoxyribonucleicAcid { data: data.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        RibonucleicAcid::new(&(self.data
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'A' => 'U',
                'T' => 'A',
                x => x,
            })
            .collect()))
    }
}
