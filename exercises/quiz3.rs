// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



trait Grade{
    fn display(&self)-> String;
}

struct NumericalGrade(f32);
struct AlphabeticalGrade(String);

impl Grade for NumericalGrade{
    fn display(&self)->String{
        self.0.clone().to_string()
    }
}

impl Grade for AlphabeticalGrade{
    fn display(&self)->String{
        self.0.clone()
    }
}


struct ReportCard<T: Grade> {
    grade: T,
    student_name: String,
    student_age: u8,
}

impl<T: Grade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade.display())
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: NumericalGrade(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: AlphabeticalGrade(String::from("A+")),
            student_name: "Gary Plotter".to_string(),
            student_age: 11
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
