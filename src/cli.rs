use crate::model::Token;

pub struct Parameters {
    pub jwt: Token,
}

impl Parameters {
    pub fn new(args: &[String]) -> Result<Parameters, &'static str> {
        if args.len() < 2 {
            return Err("Missing argument (JWT)");
        }
        let jwt_string = args[1].clone();

        let parts: Vec<&str> = jwt_string.split(".").collect();
        assert_eq!(parts.len(), 3, "Provided argument is not well formed");

        let jwt = Token::new(jwt_string);

        Ok(Parameters { jwt })
    }
}
