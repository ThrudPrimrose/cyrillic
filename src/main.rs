mod map;
extern crate clap;

use crate::map::map::FinalStates;
use clap::{Arg,App};

use std::fs;
use std::path::Path;

fn ends_with_mp3(s: &str) -> bool {
    let size = s.len();
    let chars = s.get(size-3..size);
    let mp3: &str = "mp3";
    match chars {
        Some(s) => if s.eq(mp3) {
            return true
        }else{
            return false
        }
        _ => return false
    }
}

fn iterate_files(path : &str, fs: &FinalStates<'static>) {
    let ret = fs::read_dir(path);
    
    match ret {
        Err(_) => {println!("No such path"); return}, 
        Ok(_) => {
            let paths = ret.unwrap();

            for p in paths {
                let file = p.unwrap();
                let res = file.file_name().into_string();
                match res {
                    Ok(s) => {
                        if ends_with_mp3(&s) {
                            //println!("{} is an mp3 file", s);
                            if fs.convertible(&s) {
                                //println!("{} is in cyrillic", s);
                                //convert cyrillic to latin
                                let converted = fs.convert(&s);
                                println!("{} -> {}",s, converted);
                                let namelen = s.len();
                                //get full path
                                let mut samepath = file.path().into_os_string().into_string().unwrap();
                                //remove file name
                                let cullafter = samepath.len() - namelen;
                                let opt = samepath.get(0..cullafter);
                                //assign culled path 
                                match opt {
                                    Some(ss) => samepath = ss.to_string(),
                                    None => samepath = "".to_string()
                                }
                                
                                //generate both paths again
                                let mut old = samepath.clone();
                                old.push_str(&s);
                                let oldp = Path::new(&old);
                                samepath.push_str(&converted);
                                let curp = Path::new(&samepath);
                                println!("{} -> {}", old, samepath);

                                //change names
                                let _changed = fs::rename(oldp,curp);
                                //println!("{:?}",changed);
                            }
                        }
                    }
                    Err(_) => println!("filename not valid?")
                }
            }
            return
        }
    }
}

fn main() {
    let args = App::new("Cyrillic to Latin")
                .version("0.1")
                .arg(Arg::with_name("path")
                    .short('p')
                    .long("path")
                    .value_name("path")
                    .takes_value(true)
                    .required(false))
                .get_matches();
    
    let val = args.value_of("path");
    let mappings = map::map::FinalStates::new();
    match val {
        None => iterate_files(".", &mappings),
        Some(s) => iterate_files(s, &mappings)
    }

}
