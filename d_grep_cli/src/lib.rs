use std::fs;

pub struct Config {
    pub flag: String,
    pub files_path: String,
    pub search: String,
    pub case_ignore: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 1 || args.len() > 4 {
            return Err("Invalid argument count");
        }

        let flag = String::from(&args[1]);
        let mut case_ignore: bool = false;

        if flag == "-h" || flag == "-help" {
            print!(
                "
                CLI Tool for search in files. \r\n
                -h or -heplp for this info message \r\n
                -i or --ignore-case for ignore search case \r\n
                "
            );
            return Err("");
        }

        if flag == "-i" || flag == "--ignore-case" {
            case_ignore = true;
            return Ok(Self {
                flag,
                files_path: args[1].clone(),
                search: args[2].clone(),
                case_ignore,
            });
        }

        let files_path = args[1].clone();
        let search = args[2].clone();

        Ok(Self {
            flag,
            files_path,
            search,
            case_ignore,
        })
    }

    pub fn find(&self) -> Vec<String> {
        let rows = fs::read_to_string(&self.files_path).expect("Unable to open file");
        let mut res = Vec::new();
        let query = &self.search;

        for line in rows.lines() {
            if self.case_ignore {
                if line.to_lowercase().contains(query) {
                    res.push(line.to_string());
                }
            } else {
                if line.contains(query) {
                    res.push(line.to_string());
                }
            }
        }

        res
    }
}
