//extern crate clipboard;

use std::{convert::TryInto, string::ToString, fmt};
use structopt::StructOpt;
//use clipboard::{ClipboardProvider, ClipboardContext};

enum ErrorCode {
    BaseConversionErr,
    TargetBaseErr,
    InputBaseErr,
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            ErrorCode::BaseConversionErr => "Base Conversion Error",
            ErrorCode::TargetBaseErr     => "Target Base Error",
            ErrorCode::InputBaseErr      => "Input Base Error",
        })
    }
}

fn main() -> Result<(), ErrorCode> {
    // Get args
    let opt = Opt::from_args();

    if opt.verbosity > 0 {
        println!("{:?}", opt);
    }

    let bases: Vec<String> = if opt.bases.is_empty() {
        vec![
            "2" .to_string(),
            "8" .to_string(),
            "10".to_string(),
            "16".to_string()
        ]
    }
    else {
        opt.bases
    };

    // Convert input number to base 10
    let num = match u128::from_str_radix(&opt.num, opt.base) {
        Ok(v)  => v,
        Err(_e) => {
            println!("Could not convert {} from base {}", opt.num, opt.base);
            return Err(ErrorCode::BaseConversionErr);
        },
    };

    // Print conversions
    for target_base in bases {
        let custom_base = match u32::from_str_radix(&target_base, 10) {
            Ok (v) => v,
            Err(_) => {
                println!("Error with target base {}\nPlease provide target base is base 10.", target_base);
                return Err(ErrorCode::TargetBaseErr);
            },
        };
        let mut out_str = match as_string_base(&num, custom_base) {
            Ok(v)  => v,
            Err(e) => {
                println!("Error with custom base:\n\t{}", e);
                return Err(ErrorCode::InputBaseErr);
            },
        };

        if !opt.silent {
            if !opt.no_sep && opt.sep_length > 0 {
                // Pad string every opt.spacer_length characters
                // Need size-1/spacer_len additional slots in the string
                let mut insert_idx: i32 = out_str.len() as i32 - opt.sep_length as i32;
                while insert_idx > 0 {
                    let left  = String::from(&out_str[..(insert_idx as usize)]);
                    let right = String::from(&out_str[(insert_idx as usize)..]);
                    out_str = left;
                    out_str.push(opt.sep_char);
                    out_str.push_str(&right);
                    insert_idx -= opt.sep_length as i32;
                }
            }
            if !opt.bare {
                print!("Base {:02}: ", &custom_base);
            }
            println!("{}", out_str);
        }

//        if opt.copy {
//            let mut xcb: ClipboardContext = match ClipboardProvider::new() {
//                Ok (v) => v,
//                Err(e) => {
//                    println!("Error getting clipboard provider: {}", e);
//                    return;
//                },
//            };
//
//            match xcb.set_contents(out_str) {
//                Ok (_v) => (),
//                Err(e) => {
//                    println!("Error copying to clipboard:\n\t{}", e);
//                },
//            }
//        }
    }
    return Ok(());
}

fn as_string_base(num: &u128, base: u32) -> Result<String, String> {
    if base<2 || base>33 {
        Err(String::from("Invalid Base.  Base must be between 1 and 33 (i.e. 2 to 32)"))
    }
    else {
        let mut str_num = String::new();

        let mut tmp: u128 = *num;
        let mut count: u32 = 0;

        while tmp > 0 {
            let radix_mask: u128 = u128::from((base as u128).pow(count));
            let digit: u8 = match ((tmp / radix_mask) % u128::from(base)).try_into() {
                Ok(v)  => v,
                Err(_) => {
                    return Err(format!("Error while trying to convert to radix {}", base));
                },
            };

            let ch = if digit >= 10 {
                (b'A' + (digit-10)) as char
            }
            else {
                (b'0' + digit) as char
            };

            str_num = ch.to_string() + str_num.as_str();

            count += 1;
            tmp -= u128::from(digit) * radix_mask;
        }

        Ok(str_num)
    }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "numconverter", about = "A CLI number conversion utility written in Rust")]
struct Opt {
    /// Pad the output with leading 0s
    #[structopt(short, long, default_value = "0")]
    pad: u8,

    /// Put a spacer every N characters
    #[structopt(short, long, default_value = "4")]
    sep_length: u32,

    /// Specify spacer char
    #[structopt(long, default_value = "_")]
    sep_char: char,

    /// Do not pad the output
    #[structopt(long)]
    no_sep: bool,

    /// Input Base
    #[structopt(short, long, default_value = "10")]
    base: u32,

    /// Copy to system clipboard
    #[structopt(short, long)]
    copy: bool,

    /// Do not print output (for use with clipboard)
    #[structopt(short, long)]
    silent: bool,

    /// Pretty Print
    #[structopt(long)]
    bare: bool,

    /// Verbosity (more v's, more verbose)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,

    /// Number to convert
    num: String,

    /// Bases to convert to
    bases: Vec<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin() {
        assert_eq!(as_string_base(&4,   2).unwrap(), "100");
        assert_eq!(as_string_base(&12,  2).unwrap(), "1100");
        assert_eq!(as_string_base(&187, 2).unwrap(), "10111011");
        assert_eq!(as_string_base(&69,  2).unwrap(), "1000101");
    }

    #[test]
    fn test_hex(){
        assert_eq!(as_string_base(&4,   16).unwrap(), "4");
        assert_eq!(as_string_base(&12,  16).unwrap(), "C");
        assert_eq!(as_string_base(&187, 16).unwrap(), "BB");
        assert_eq!(as_string_base(&69,  16).unwrap(), "45");
    }
}
