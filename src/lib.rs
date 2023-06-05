pub mod parser {
    use std::env;
    use std::error::Error;
    use std::fs;
    use std::process;
    struct Config {
        query: String,
        filename: String,
    }
    impl Config {
        fn parse_config() -> Result<Config, &'static str> {
            let args: Vec<String> = env::args().collect();
            if args.len() < 3 {
                return Err("输入");
            }
            let query = args[1].to_string();
            let filename = args[2].to_string();
            Ok(Config { query, filename })
        }
    }
    pub fn run() -> Result<(), Box<dyn Error>> {
        let config: Config = Config::parse_config().unwrap_or_else(|err| {
            println!("problem parsing args, {}", &err);
            process::exit(1);
        });
        let contents = fs::read_to_string(&config.filename)?;
        println!("query = {}\nfilename = {}", &config.query, &config.filename);
        println!("内容为{}", &contents);
        return Ok(());
    }
}
