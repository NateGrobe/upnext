extern crate colored;
use colored::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use chrono::prelude::*;
use chrono::Month;
use num_traits::FromPrimitive;


#[derive(Debug, Clone)]
struct Assignment {
    course: String,
    name: String,
    date: Option<Date<chrono::Utc>>
}

impl Assignment {
    pub fn new(course: &String, content: String) -> Assignment {
        if content.contains('-') {
            // parse date
            let content_split: Vec<&str> = content.as_str().split('-').collect();
            let date_split: Vec<&str> = content_split[1].trim().split(' ').collect();

            // determine month from string
            let mut month = Month::January;
            loop {
                if month.name() == date_split[0].trim() {
                    break;
                }
                month = month.succ();
            }

            // get values for day month year
            let month = month.number_from_month();
            let year = Utc::now().year();
            let day = date_split[1].trim().parse::<u32>().unwrap();

            let date = Utc.ymd(year, month, day);

            return Assignment {
                course: course.clone(),
                name: content_split[0].trim().to_string(),
                date: Some(date)
            }
        }

        Assignment {
            course: course.clone(),
            name: content,
            date: None,
        }
    }

    // return date to string form and return string representing assignment
    pub fn format_assignment(&self) -> String {
        let course = match self.course.as_str() {
            "DBMS:" => self.course.green(),
            "Psych:" => self.course.magenta(),
            "Micro:" => self.course.cyan(),
            "Architecture:" => self.course.blue(),
            "Systems:" => self.course.red(),
            "Life:" => self.course.yellow(),
            _ => self.course.bold()
        };

        if self.date == None {
            format!("{} {}\n", self.course, self.name);
        }

        let date = self.date.unwrap();
        let month = Month::from_u32(date.month()).unwrap();
        let weekday = date.weekday();
        let day = date.day();
        let today = Utc::now();

        // checks to see if assignment is due today and if so change date to "Today"
        if Month::from_u32(today.month()).unwrap() == month && today.day() == day && today.weekday() == weekday {
            format!("{} {} - {}", course, self.name, "Today".red())
        } else {
            format!("{} {} - {} {:?} {}", course, self.name, weekday, month, day)
        }
    }
}

fn main() -> io::Result<()> {

    // read file in BufReader
    let file_path = String::from("/home/nategrobe/todo.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // collect lines into vector
    let lines: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

    // instantiate loop variables
    let mut assignments: Vec<Assignment> = Vec::new();
    let line_iter = lines.iter();
    let mut current_course: Option<String>= None;

    // generate assignments from file
    for line in line_iter {
        if line.len() == 0 {
            continue;
        }

        if line.contains(':') {
            current_course = Some(line.clone());
            continue;
        }

        let assignment = Assignment::new(current_course.as_ref().unwrap(), line.clone());
        assignments.push(assignment);

    }

    // sort assignments by date
    assignments.sort_by_key(|a| a.date);

    // display all assignments starting with ones that have dates
    for a in assignments.iter() {
        if a.date != None {
            println!("{}\n", a.format_assignment());
        }
    }

    for a in assignments.iter() {
        if a.date == None {
            println!("{} {}\n", a.course.yellow(), a.name);
        }
    }

    Ok(())
}
