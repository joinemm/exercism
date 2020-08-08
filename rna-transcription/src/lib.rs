#[derive(Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct DNA {
    dna: String,
}

#[derive(Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct RNA {
    rna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut dna_object = Self {
            dna: "".to_string(),
        };
        for (i, c) in dna.chars().enumerate() {
            match c {
                'G' | 'C' | 'T' | 'A' => dna_object.dna.push(c),
                _ => return Err(i),
            }
        }
        Ok(dna_object)
    }

    pub fn into_rna(self) -> RNA {
        let mut rna: String = String::new();
        for c in self.dna.chars() {
            match c {
                'G' => rna.push('C'),
                'C' => rna.push('G'),
                'T' => rna.push('A'),
                'A' => rna.push('U'),
                _ => panic!("something went terribly wrong"),
            }
        }
        RNA { rna: rna }
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut rna_object = Self {
            rna: "".to_string(),
        };
        for (i, c) in rna.chars().enumerate() {
            match c {
                'C' | 'G' | 'A' | 'U' => rna_object.rna.push(c),
                _ => return Err(i),
            }
        }
        Ok(rna_object)
    }
}
