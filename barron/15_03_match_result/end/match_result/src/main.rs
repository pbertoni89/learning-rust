use std::fs;
use std::io;

fn rustrover_suggests(result: Result<String, io::Error>) -> String {
    let contents = result.unwrap_or_else(|error| match error.kind() {
        io::ErrorKind::NotFound => String::from("File not found."),
        io::ErrorKind::PermissionDenied => String::from("Permission denied."),
        _ => panic!("Another type of error: {:?}", error)
    });
    contents
}

fn main() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    //noinspection RsReplaceMatchExpr
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of error: {:?}", error)
        }
    };

    let result = fs::read_to_string("yet_another.txt");
    rustrover_suggests(result);

    println!("contents is: {:?}", contents);
}