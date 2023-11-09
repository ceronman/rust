#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        if std::env::var_os("MY_DEBUG").is_some() {
            let t = format!($($arg)*)
                .replace("#black ", "\x1b[30m")
                .replace("#red ", "\x1b[31m")
                .replace("#green ", "\x1b[32m")
                .replace("#yellow ", "\x1b[33m")
                .replace("#blue ", "\x1b[34m")
                .replace("#magenta ", "\x1b[35m")
                .replace("#cyan ", "\x1b[36m")
                .replace("#white ", "\x1b[36m")

                .replace("#BLACK ", "\x1b[90m")
                .replace("#RED ", "\x1b[91m")
                .replace("#GREEN ", "\x1b[92m")
                .replace("#YELLOW ", "\x1b[93m")
                .replace("#BLUE ", "\x1b[94m")
                .replace("#MAGENTA ", "\x1b[95m")
                .replace("#CYAN ", "\x1b[96m")
                .replace("#WHITE ", "\x1b[97m")

                .replace("#bold ", "\x1b[1m")
                .replace("#italic ", "\x1b[3m")

                .replace("#end", "\x1b[0m");
            println!("{}\x1b[0m", t);
        }
    }};
}
