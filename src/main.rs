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
use std::path::PathBuf;
use clap::Parser;
use color_eyre::Result;

mod util;
mod renderer;

#[derive(Parser, Debug, Clone)]
#[command(name = "svg2colored-png")]
#[command(author = "MCorange <mcorangecodes@gmail.com>")]
#[command(version = "1.0.2")]
#[command(about = "Converts svgs to multiple png's that differ in color", long_about = "Made by MCorange <mcorangecodes@gmail.com>")]
pub struct Args {
    /// Input folder with the SVG's
    #[arg(long, short)]
    input_folder: PathBuf,

    /// Output folder where the PNG's will be placed
    #[arg[long, short]]
    output_folder: PathBuf,

    /// Comma seperated colors that will be used in HEX
    #[arg[long, short, default_value_t = String::from("0d6efd,6c757d,198754,0dcaf0,ffc107,dc3545,f8f9fa,212529,ffffff,000000")]]
    colors: String,

    /// Width of the generated PNG's
    #[arg(long, default_value_t = 1024)]
    width: u32,

    /// Height of the generated PNG's
    #[arg(long, default_value_t = 1024)]
    height: u32

}



fn main() -> Result<()> {
    let args = Args::parse();
    
    let r = renderer::Renderer::new(args.clone())?;

    println!(concat!(
        "svg2colored-png Copyright (C) 2023 MCorange<mcorangecodes@gmail.com>\n",
        "This program comes with ABSOLUTELY NO WARRANTY.\n",
        "This is free software, and you are welcome to redistribute it\n",
        "under certain conditions.\n",
    ));

    for e in std::fs::read_dir(args.input_folder.clone())? {
        let entry = e?;
        let path = entry.path();
        if path.is_dir() {
            util::logger::info(&format!("Skipping folder '{}' since folder walking is not yet implemented", path.clone().display()));
        } else {
            match r.render(path.clone(), args.output_folder.clone()){
                Ok(_) => util::logger::info(&format!("Successfully rendered all colors of '{}'", path.clone().display())),
                Err(_) => util::logger::error(&format!("Failed to render '{}'", path.clone().display()))
            };
        }
    }
    


    Ok(())
}


