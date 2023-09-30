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

mod renderer;


#[derive(Parser, Debug, Clone)]
#[command(name = "svg2colored-png")]
#[command(author = "MCorange <mcorangecodes@gmail.com>")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Converts svgs to multiple png's that differ in color", long_about = "Made by MCorange <mcorangecodes@gmail.com>")]
pub struct Args {
    /// Input folder with the SVG's
    #[arg(long, short)]
    input: PathBuf,

    /// Output folder where the PNG's will be placed
    #[arg[long, short]]
    output: PathBuf,

    /// Comma seperated colors that will be used in HEX Eg. 000000,ffffff
    /// Can be like an object: black:000000,white:ffffff
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
    simple_logger::SimpleLogger::new().env().init()?;

    let args = Args::parse();
    
    let mut r = renderer::Renderer::new(&args)?;

    println!(concat!(
        "svg2colored-png Copyright (C) 2023 MCorange<mcorangecodes@gmail.com>\n",
        "This program comes with ABSOLUTELY NO WARRANTY.\n",
        "This is free software, and you are welcome to redistribute it\n",
        "under certain conditions.\n",
    ));


    let p = &args.input.clone();

    if !p.exists() {
        log::error!("The File/Folder {:?} doesnt exist!", p);
        return Ok(());
    }

    if p.is_file() {
        match r.render(&p, &args) {
            Ok(_) => log::info!("Successfully rendered all colors of {p:?}"),
            Err(e) => {
                log::error!("Failed to render {p:?}: {}", e)
            }
        }
    } else {
        // let files = 
        for f in std::fs::read_dir(p)? {
            match f {
                Ok(f) => {
                    let f = f.path();
                    if f.is_dir() {
                        log::warn!("Skipping folder '{f:?}' since folder walking is not yet implemented");
                        continue;
                    }

                    match r.render(&f, &args) {
                        Ok(_) => log::info!("Successfully rendered all colors of {f:?}"),
                        Err(e) => {
                            log::error!("Failed to render {f:?}: {e:?}");
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to read file {p:?}: {e:?}");
                },
            }
        }
    }
    log::info!("Done! Rendered {} files.", r.count);

    Ok(())
}


