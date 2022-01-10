use anyhow::{bail, ensure, Result};

use clap::Parser;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Parser, Debug)]
#[clap(name = "RPN Program", about, version = "1.10.1", author = "Your Name")]
struct Args {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    fomura_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.fomura_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        let _ = run(reader, args.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        let _ = run(reader, args.verbose);
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()>{
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();

        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => println!("{:#?}", e)   
        }
    }

    Ok(())
}

struct RpnCalculator(bool);
impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formura: &str) -> Result<i32> {
        let mut tokens = formura.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };

                stack.push(res);
            }
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        
        ensure!(stack.len() == 1, "invalid syntax");
        
        Ok(stack[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        let _ = calc.eval("1 1 ^");
    }
}