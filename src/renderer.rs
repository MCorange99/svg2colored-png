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
use usvg_text_layout::{fontdb::{self, Database}, TreeTextToPath};
use usvg::{self, Size};
use resvg;
use tiny_skia;
use color_eyre::Result;

use crate::util::{logger, self};

pub struct Renderer {
    fontdb: Database,
    colors: Vec<String>,
    size: (u32, u32)
}

impl Renderer {
    pub fn new(args: crate::Args) -> Result<Self> {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();

        let colors = args.colors.split(",").map(|s| {
            s.to_string()
        })
        .collect::<Vec<String>>();

        for color in colors.clone() {
            std::fs::create_dir_all(args.output_folder.join(color))?;
        }

        Ok(Self {
            fontdb: db,
            colors: colors,
            size: (args.width, args.height)
        })
    }

    pub fn render(&self, path_in: PathBuf, out_dir: PathBuf) -> Result<(), ()> {
        let svg_data = match std::fs::read_to_string(path_in.clone()) {
            Ok(d) => d,
            Err(_) => return Err(logger::error(&format!("File {} does not exist", path_in.clone().display())))
        };

        let mut opt = usvg::Options::default();
        // Get file's absolute directory.
        opt.resources_dir = std::fs::canonicalize(path_in.clone())
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));
        opt.default_size = Size::new(1000.0, 1000.0).unwrap();
        opt.dpi = 200.0;

        for color in self.colors.clone() { 
            let color = color.replace("#", "");
            let svg_data_bytes = svg_data.replace("fill=\"currentColor\"", &format!("fill=\"#{}\"", color));
            let svg_data_bytes = svg_data_bytes.as_bytes();
            //fill="currentColor"
            let mut tree = usvg::Tree::from_data(&svg_data_bytes, &opt).unwrap();
            tree.convert_text(&self.fontdb);
    
    
            let mut pixmap = tiny_skia::Pixmap::new(self.size.0, self.size.1).unwrap();
            // let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
            resvg::render(
                &tree,
                usvg::FitTo::Size(self.size.0, self.size.1),
                tiny_skia::Transform::default(),
                pixmap.as_mut(),
            )
            .unwrap();
            let f_n = path_in.clone().file_name().unwrap().to_string_lossy().replace(".svg", ".png");
            let p = out_dir
                                .join(color.clone())
                                .join(
                                    f_n
                                );
            util::logger::info(format!("Rendering '{}'", p.display()));
            pixmap.save_png(p).unwrap();

        }

        Ok(())
    }
}

