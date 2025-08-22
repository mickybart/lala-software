use crate::punchclock::{ParsedEntries, ParsedEntry, Report};

pub(crate) struct Myr {
    input: String,
}

impl Myr {
    pub(crate) fn new(csv: String) -> Self {
        Self { input: csv }
    }
}

impl Report for Myr {
    fn parser(&self) -> ParsedEntries {
        let file = std::fs::File::open(&self.input).unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(file);

        let mut entries: Vec<ParsedEntry> = vec![];

        let mut day: String = Default::default();
        let mut group: String = Default::default();
        let mut employee: String = Default::default();

        for result in rdr.records().skip(3) {
            let record = result.expect("a CSV record");

            match record.len() {
                1 => {
                    // New day
                    day = record.get(0).unwrap().get(0..10).unwrap().to_owned();
                }
                10 => {
                    // Is it employee total ?
                    if record.get(6).unwrap() == "TOTAUX DE L'EMPLOYÉ" {
                        // We have all necessary information for the current employee (day, group, total)
                        let total = record.get(7).unwrap().to_owned();

                        let entry = ParsedEntry {
                            day: day.clone(),
                            group: group.clone(),
                            employee: employee.clone(),
                            total,
                        };

                        /*
                           MYR report can provide multiple total for the same employee and same day
                           if the employee punch/depunch multiple time during the day.

                           eg:

                           "2025-08-14 00:00:00 au 2025-08-14 23:59:59"
                           Employés,"A, L",08:03,15:27,7.40,0.00,0.00,7.40,119.14,

                           ,,,,,,"TOTAUX DE L'EMPLOYÉ",7.40,119.14,0.00
                           ,"A, L",15:28,15:28,0.00,0.00,0.00,0.00,0.00,

                           To avoid it, we need to get the latest entry to be sure that we are not adding a duplicate entry
                        */
                        let last_entry = entries.last();

                        match last_entry {
                            Some(last_entry)
                                if last_entry.day == day && last_entry.employee == employee =>
                            {
                                // skip
                                ()
                            }
                            _ => entries.push(entry),
                        }

                        // println!("{};{};{};{}", day, group, employee, record.get(7).unwrap());
                    } else {
                        // New group ?
                        let record_group = record.get(0).unwrap();
                        if !record_group.is_empty() {
                            group = record_group.to_owned();
                        }

                        // New employee ?
                        let record_employee = record.get(1).unwrap();
                        if !record_employee.is_empty() {
                            employee = record_employee.to_owned();
                        }
                    }
                }
                _ => (),
            }
        }

        ParsedEntries(entries)
    }
}

#[cfg(test)]
mod test {
    use std::{fmt::Write, io::Read};

    use crate::punchclock::{Myr, Report};

    #[test]
    fn test_parser() {
        // Load test data and parse them
        let myr = Myr::new("res/punch-test-data.csv".to_owned());
        let entries = myr.parser();

        assert_eq!(entries.len(), 52);

        let mut result = String::new();
        write!(&mut result, "{}", entries).unwrap();

        // load test data to compare result with expected result
        let mut f = std::fs::File::open("res/punch-test-result.csv").unwrap();
        let mut expected_result = String::new();
        f.read_to_string(&mut expected_result).unwrap();

        assert_eq!(result, expected_result);
    }
}