pub mod commands;
pub mod ci_cd;

pub fn some_task() {
    println!("some task")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        some_task()
    }
}
