pub struct arguments {
    key: String,
    value: String,
}

pub fn parse_args(args: Vec<String>) -> Vec<arguments> {
    let mut res = Vec::new();
    let mut i = 1;
    while i < args.len() {
        let key = args[i].clone();
        let value = args[i + 1].clone();
        res.push(arguments { key, value });
        i += 2;
    }
    res
}
