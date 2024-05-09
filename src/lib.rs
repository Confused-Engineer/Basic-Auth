use Ini::Ini;


struct Config {
    Users: Vec<String>,
}





pub fn Validate(user: String, password: String) -> bool
{
    
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
