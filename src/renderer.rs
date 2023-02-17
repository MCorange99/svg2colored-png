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
use usvg;
use resvg;
use tiny_skia;
use color_eyre::Result;

use crate::{util::{logger, self}, Args};

pub struct Renderer {
    fontdb: Database,
    colors: Vec<String>,
    colors_obj: Vec<Vec<String>>,
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
        
        let colors_obj = if args.colors_object.is_empty() {
            Self::init_folders(colors.clone(), args.clone())?;
            Vec::new()
        } else {
            let colors_obj = args.colors_object.split(",").map(|s| {
                s.to_string()
            })
            .collect::<Vec<String>>();

            let colors_obj = colors_obj.iter().map(|p| {
                let s: Vec<&str> = p.split(":").collect();
                vec![ s[0].to_string(), s[1].to_string() ]
            }).collect::<Vec<Vec<String>>>();

            let folders = colors_obj.iter().map(|v| {
                v[0].clone()
            }).collect::<Vec<String>>();
            Self::init_folders(folders, args.clone())?;
            colors_obj
        };

        Ok(Self {
            fontdb: db,
            colors_obj,
            colors,
            size: (args.width, args.height)
        })
    }

    pub fn render(&self, fi: PathBuf, args: crate::Args) -> Result<(), ()> {
        let ext = fi.clone();
        let ext = match ext.extension() {
            Some(e) => e,
            None => return Err(util::logger::warning(format!("File '{}' is not of SVG type", fi.clone().to_str().unwrap()))),
        };
        
        if ext.to_str().unwrap() != "svg" {
            return Err(util::logger::warning(format!("File '{}' is not of SVG type", fi.clone().to_str().unwrap())));
        }

        if self.colors_obj.is_empty() {   
            for color in self.colors.clone() { 
                let fo = self.get_out_file(fi.clone(), color.clone(), args.clone());
                self.render_one(fi.clone(), fo, color.clone())?;    
                
            }
        } else {
            for o in self.colors_obj.clone() { 
                let color = &o[1];
                let name = &o[0];
                let fo = self.get_out_file(fi.clone(), name.clone(), args.clone());
                self.render_one(fi.clone(), fo, color.clone())?;
                
            }
        }

        Ok(())
    }


    fn render_one(&self, fi: PathBuf, fo: PathBuf, color: String) -> Result<(), ()>{

        if fo.exists() {
            util::logger::warning(format!("File '{}' exists, skipping", fo.to_str().unwrap()));
            return Ok(());
        }

        let svg = self.get_svg_data(fi.clone())?;
        let svg = self.set_color(svg, color);

        let mut opt = usvg::Options::default();
        // Get file's absolute directory.
        opt.resources_dir = std::fs::canonicalize(fi.clone())
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));

        let mut tree = match usvg::Tree::from_data(svg.as_bytes(), &opt) {
            Ok(v) => Ok(v),
            Err(_) => Err(util::logger::error(format!("Failed to parse '{}'", fi.clone().display())))
        }?;

        tree.convert_text(&self.fontdb);

        let mut pixmap = tiny_skia::Pixmap::new(self.size.0, self.size.1).unwrap();

        util::logger::info(format!("Rendering '{}'", fo.display()));
            
        //? maybe handle this and possibly throw error if its none
        let _ = resvg::render(
            &tree,
            usvg::FitTo::Size(self.size.0, self.size.1),
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        );

        pixmap.save_png(fo).unwrap();

        Ok(())
    }

    fn get_svg_data(&self, fi: PathBuf) -> Result<String, ()>{
        match std::fs::read_to_string(fi.clone()) {
            Ok(d) => Ok(d),
            Err(_) => return Err(logger::error(&format!("File {} does not exist", fi.clone().display())))
        }
    }

    fn set_color(&self, svg: String, color: String) -> String {
        svg.replace("fill=\"currentColor\"", &format!("fill=\"#{}\"", color))
    }

    fn get_out_file(&self, fi: PathBuf, sub_folder: String, args: crate::Args) -> PathBuf {
        let mut fo = args.output_folder.clone();
        fo.push(sub_folder);
        fo.push(fi.clone().file_name().unwrap());
        fo.set_extension("png");
        fo
    }

    fn init_folders(folders: Vec<String>, args: Args) -> Result<()>{
        for folder in folders.clone() {
            std::fs::create_dir_all(args.output_folder.join(folder))?;
        }
        Ok(())
    }
}

