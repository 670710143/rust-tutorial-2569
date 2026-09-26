
//Option <T> enum มาตรฐานของ Rust แก้ปัญหาค่า Null

fn main() {
    println!("=========== Grade Checker ===========");
    let scores = [85,50,67,150];
    for score in scores{
        let grade = get_grade(score);
        println!("{} points : {}",score,result(grade));

    }
}
fn get_grade(score: i32) -> Option <char>{
    match score{
        80..=100 => Some('A'),
        70..=79 => Some('B'),
        60..=69 =>Some('C'),
        0..=59 => Some('D'),
        _ => None,
    }
}
fn result(grade: Option<char>) -> String{
    match grade{
        Some(g) => format!("Your grade is {}", g), // varient ที่มีค่า -> ดึงออกมาใช้ต่อ
        None => format!("Invalid score."),         // varient ไม่มีค่า
    }

}
