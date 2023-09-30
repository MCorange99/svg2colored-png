use std::path::{Path, PathBuf};

use clap::builder::OsStr;
use color_eyre::{Result, eyre::bail};
use usvg_text_layout::TreeTextToPath;

use crate::Args;

#[derive(Debug, Clone)]
enum ColorType {
    Array(Vec<String>),
    Object(Vec<(String, String)>),
    None
}


#[derive(Debug, Clone)]
pub struct Renderer {
    fontdb: usvg_text_layout::fontdb::Database,
    colors: ColorType,
    size: (u32, u32),
    pub count: u64,
}

impl Renderer {
    pub fn new(args: &Args) -> Result<Self> {
        let mut db = usvg_text_layout::fontdb::Database::new();
        db.load_system_fonts();

        let mut this = Self {
            fontdb: db,
            colors: ColorType::None,
            size: (args.width, args.height),
            count: 0,
        };

        let colors = if args.colors.contains(':') {
            //? object
            let obj = args.colors.split(',').map(|s| {
                let s = s.split(':').collect::<Vec<&str>>();

                if s.len() < 2 {
                    log::error!("Invalid color object, try checking help");
                    return None;
                }

                Some((s[0].to_string(), s[1].to_string()))
            }).collect::<Vec<Option<(String, String)>>>();

            let mut colors = Vec::new();

            for c in obj {
                if let Some(c) = c {
                    std::fs::create_dir_all(args.output.join(&c.0))?;
                    
                    colors.push(c);
                }
            }

            ColorType::Object(colors)

        } else {
            //? list
            // let colors = args.colors.split(",").map(|s| {
            //     s.to_string()
            // })
            // .collect::<Vec<String>>();
            
            let mut colors = Vec::new();

            for color in args.colors.split(',') {
                std::fs::create_dir_all(args.output.join(&color))?;
                colors.push(color.to_string())
            }
            
            
            ColorType::Array(colors)
        };

        this.colors = colors;
        Ok(this)
    }


    pub fn render(&mut self, fi: &Path, args: &Args) -> Result<()> {
        match fi.extension() {
            Some(e) if e.to_str() == Some("svg") => {},
            Some(_) |
            None => {
                log::warn!("Filer {:?} is not of type SVG", fi);
                // util::logger::warning(format!("File '{}' is not of SVG type", fi.clone().to_str().unwrap()));
                bail!("Failed to render");
            }
        };

        match self.colors.clone() {
            ColorType::Array(c) => {
                for color in c { 
                    log::info!("Rendering the color {color:?}");
                    let fo = self.get_out_file(fi, &color, &args);
                    self.render_one(fi, &fo, &color)?;    
                    
                }
            },
            ColorType::Object(c) => {
                for o in c {
                    log::info!("Rendering the color {:?}", o);
                    let fo = self.get_out_file(fi, &o.0, &args);
                    self.render_one(fi, &fo, &o.1)?;
                    
                }
            },
            ColorType::None => unreachable!(),
        }

        Ok(())
    }

    fn render_one(&mut self, fi: &Path, fo: &Path, color: &String) -> Result<()>{

        if fo.exists() {
            log::warn!("File {fo:?} exists, skipping");
            return Ok(());
        }

        let svg = self.set_color(&self.get_svg_data(fi)?, &color);

        let mut opt = usvg::Options::default();
        // Get file's absolute directory.
        opt.resources_dir = std::fs::canonicalize(fi.clone())
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));

        let mut tree = match usvg::Tree::from_data(svg.as_bytes(), &opt) {
            Ok(v) => v,
            Err(_) => {
                log::error!("Failed to parse {fi:?}");
                bail!("");
            }
        };

        tree.convert_text(&self.fontdb);

        let mut pixmap = tiny_skia::Pixmap::new(self.size.0, self.size.1).unwrap();

        log::info!("Rendering {fo:?}");
            
        //? maybe handle this and possibly throw error if its none
        let _ = resvg::render(
            &tree,
            usvg::FitTo::Size(self.size.0, self.size.1),
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        );


        pixmap.save_png(fo)?;
        self.count += 1;
        Ok(())
    }


    #[inline]
    fn get_out_file(&mut self, fi: &Path, sub_folder: &String, args: &crate::Args) -> PathBuf {
        let mut fo: std::path::PathBuf = args.output.clone();
        fo.push(sub_folder);
        fo.push(fi.file_name().unwrap_or(&OsStr::from("default")).to_str().unwrap_or("default").replace(".svg", ""));
        fo.set_extension("png");
        fo
    }

    fn set_color(&self, svg: &String, color: &String) -> String {
        svg.replace("fill=\"currentColor\"", &format!("fill=\"#{}\"", color))
    }

    fn get_svg_data(&self, fi: &Path) -> Result<String>{
        match std::fs::read_to_string(fi) {
            Ok(d) => Ok(d),
            Err(_) => {
                log::error!("File {fi:?} does not exist");
                bail!("File {fi:?} does not exist");
            }
        }
    }
}