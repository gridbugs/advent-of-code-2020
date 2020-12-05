use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
struct PassportCandidate {
    fields: BTreeMap<String, String>,
}

impl PassportCandidate {
    fn is_valid(&self) -> bool {
        let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        if !required_fields.iter().all(|&f| self.fields.contains_key(f)) {
            return false;
        }
        if self.fields.contains_key("cid") {
            if self.fields.len() != 8 {
                return false;
            }
        } else {
            if self.fields.len() != 7 {
                return false;
            }
        }
        if let Some(byr) = self.fields.get("byr").unwrap().parse::<u32>().ok() {
            if byr < 1920 || byr > 2002 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(iyr) = self.fields.get("iyr").unwrap().parse::<u32>().ok() {
            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(eyr) = self.fields.get("eyr").unwrap().parse::<u32>().ok() {
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(cm) = self.fields.get("hgt").unwrap().strip_suffix("cm") {
            if let Some(cm) = cm.parse::<u32>().ok() {
                if cm < 150 || cm > 193 {
                    return false;
                }
            } else {
                return false;
            }
        } else if let Some(inc) = self.fields.get("hgt").unwrap().strip_suffix("in") {
            if let Some(inc) = inc.parse::<u32>().ok() {
                if inc < 59 || inc > 76 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        let hcl = self.fields.get("hcl").unwrap();
        if !Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(hcl) {
            return false;
        }
        let ecl = self.fields.get("ecl").unwrap();
        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .all(|e| e != ecl)
        {
            return false;
        }
        if !Regex::new(r"^\d{9}$")
            .unwrap()
            .is_match(self.fields.get("pid").unwrap())
        {
            return false;
        }
        true
    }
}

impl PassportCandidate {
    fn parse_stdin() -> Vec<Self> {
        use std::io::Read;
        let mut string = String::new();
        std::io::stdin().lock().read_to_string(&mut string).unwrap();
        string
            .split("\n\n")
            .map(|s| {
                let fields = s
                    .split_whitespace()
                    .map(|f| {
                        let mut key_value = f.split(":");
                        let key = key_value.next().unwrap();
                        let value = key_value.next().unwrap();
                        (key.to_string(), value.to_string())
                    })
                    .collect::<BTreeMap<String, String>>();
                PassportCandidate { fields }
            })
            .collect()
    }
}

fn main() {
    let passport_candidates = PassportCandidate::parse_stdin();
    println!(
        "{}",
        passport_candidates.iter().filter(|p| p.is_valid()).count()
    );
}
