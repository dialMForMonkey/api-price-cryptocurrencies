
pub struct Environment(pub String);


pub trait Actions {
    fn new(name: &str ) -> Self;
    fn to_string(&self) -> String;
    fn to_list_string<'a>(&'a self) -> std::str::Split<&'a str>;
}

impl Actions for Environment {
    fn new(name: &str ) -> Self {
         let result_env = std::env::var(name);
         match result_env {
             Ok(env)=> {
                 Environment(env)
                  },
             Err(err)=> {
                 panic!("Erro ao carregar variavel {:?}",err);
             }
         }
     }

    fn to_string(&self) -> String {
        self.0.clone()
    }


    fn to_list_string<'a>(&'a self) -> std::str::Split<&'a str> {
        self.0.split("|").into_iter()
    }

}

