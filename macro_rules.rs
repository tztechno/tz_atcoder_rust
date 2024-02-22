macro_rules! input {
    ($($r:tt)*) => {
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(std::io::stdin()));
        let mut next = move |is_word: bool| -> String{
            if is_word {
                bytes
                    .by_ref()
                    .map(|r|r.unwrap() as char)
                    .skip_while(|c|c.is_whitespace())
                    .take_while(|c|!c.is_whitespace())
                    .collect()
            } else {
                bytes
                    .by_ref()
                    .map(|r|r.unwrap() as char)
                    .skip_while(|c| c == &'\n')
                    .take_while(|c| c != &'\n')
                    .collect()
            }
        };
        input_inner!{next, $($r)*};
    };
}
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, static $var:ident : $t:tt $($rest:tt)*) => {
        $var = read_value!($next, $t);
        input_inner!{$next $($rest)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($rest:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($rest)*}
    };

    ($next:expr, $var:ident : $t:tt $($rest:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($rest)*}
    };
}
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; all ]) => {
        {
            let mut str = $next(false);
            str.split_whitespace().map(|it| it.parse::<$t>().unwrap()).collect::<Vec<_>>()
        }
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len as usize).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, lines) => {
        {
            let mut vec = Vec::new();
            let mut str = $next(false);
            while str != "" {
                vec.push(str);
                str = $next(false);
            }
            vec
       }
    };

    ($next:expr, line) => {
        $next(false)
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next(true).parse::<$t>().expect("Parse error")
    };
}

fn main() {
    //067a
    unsafe {
        input! {
            a: usize, b: usize,
        }

        if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
            println!("Possible");
        } else {
            println!("Impossible");
        }
    }
}

