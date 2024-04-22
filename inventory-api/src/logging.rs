

pub fn log_vec(input: Vec<impl std::fmt::Display>) -> String {
    let mut result = String::default();
    for item in input {
        result += &format!("\t{}\n", item);
    }
    let _ = result.pop();
    result
}