#[derive(Debug)]
struct Student {
    id: i32,
    name: String,
    grades: i32,
    score: Score,
}

#[derive(Debug)]
enum Score {
    A,
    B,
    C,
    D,
}

impl Student {
    fn new(id: i32, name: String, grades: i32, score: Score) -> Student {
        Student {
            id,
            name,
            grades,
            score,
        }
    }
}

use rand::Rng;
#[allow(dead_code)]
pub fn grades() {
    let mut list: Vec<Student> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        let name = String::from("layen");
        let number = rng.gen_range(0..=100);
        let score = compute_score(number);
        let s = Student::new(i, name, number, score);
        list.push(s);
    }
    list.sort_by(|a, b| a.grades.cmp(&b.grades));
    println!("list: {:?}", list);
}

fn compute_score(n: i32) -> Score {
    match n {
        0..=60 => Score::D,
        61..=80 => Score::C,
        81..=90 => Score::B,
        _ => Score::A,
    }
}
