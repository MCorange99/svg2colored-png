/**
 *  svg2colored-png, an svg to png converter
 *  Copyright (C) 2023 MCorange<mcorangecodes@gmail.com>
 * 
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *  
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>. 
 */

pub mod color {
    #[allow(dead_code)] pub const NONE: &str = "\x1b[0m";
    #[allow(dead_code)] pub const RESET: &str = "\x1b[0m";
    #[allow(dead_code)] pub const BRIGHT: &str = "\x1b[1m";
    #[allow(dead_code)] pub const DIM: &str = "\x1b[2m";
    #[allow(dead_code)] pub const UNDERSCORE: &str = "\x1b[4m";
    #[allow(dead_code)] pub const BLINK: &str = "\x1b[5m";
    #[allow(dead_code)] pub const REVERSE: &str = "\x1b[7m";
    #[allow(dead_code)] pub const HIDDEN: &str = "\x1b[8m";
    #[allow(dead_code)] pub const FG_BLACK: &str = "\x1b[30m";
    #[allow(dead_code)] pub const FG_RED: &str = "\x1b[31m";
    #[allow(dead_code)] pub const FG_GREEN: &str = "\x1b[32m";
    #[allow(dead_code)] pub const FG_YELLOW: &str = "\x1b[33m";
    #[allow(dead_code)] pub const FG_BLUE: &str = "\x1b[34m";
    #[allow(dead_code)] pub const FG_MAGENTA: &str = "\x1b[35m";
    #[allow(dead_code)] pub const FG_CYAN: &str = "\x1b[36m";
    #[allow(dead_code)] pub const FG_WHITE: &str = "\x1b[37m";
    #[allow(dead_code)] pub const BG_BLACK: &str = "\x1b[40m";
    #[allow(dead_code)] pub const BG_RED: &str = "\x1b[41m";
    #[allow(dead_code)] pub const BG_GREEN: &str = "\x1b[42m";
    #[allow(dead_code)] pub const BG_YELLOW: &str = "\x1b[43m";
    #[allow(dead_code)] pub const BG_BLUE: &str = "\x1b[44m";
    #[allow(dead_code)] pub const BG_MAGENTA: &str = "\x1b[45m";
    #[allow(dead_code)] pub const BG_CYAN: &str = "\x1b[46m";
    #[allow(dead_code)] pub const BG_WHITE: &str = "\x1b[47m";
}

pub mod logger {
    use crate::util::color;
    #[allow(dead_code)]
    pub fn err_with_help<S: Into<String> + std::fmt::Display>(err: S, help: S) {
        println!("{rd}error{r}: {err}\n{b}help{r}: {help}", r=color::NONE, b=color::FG_BLUE, rd=color::FG_RED, help=help, err=err);
    }
    pub fn error<S: Into<String> + std::fmt::Display>(err: S) {
        println!("{rd}error{r}: {err}", r=color::NONE, rd=color::FG_RED, err=err);
    }
    pub fn info<S: Into<String> + std::fmt::Display>(info: S) {
        println!("{b}info{r}: {info}", r=color::NONE, b=color::FG_BLUE, info=info);
    }
    pub fn warning<S: Into<String> + std::fmt::Display>(text: S) {
        println!("{y}warn{r}: {text}", r=color::NONE, y=color::FG_YELLOW, text=text);
    }
}