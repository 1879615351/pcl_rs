use crate::types::{PointCloud,Point3DTrait};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
pub fn load_from_pcd<PointT:Point3DTrait>(path:&str) -> Result<PointCloud<PointT>,std::io::Error>{
    let path = Path::new(path);
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut format = String::new();
    let mut points = Vec::new();
    // 读取头部，找到数据格式和点的数量
    while reader.read_line(&mut line)? > 0 {
        if line.starts_with("POINTS"){

        }
        if line.starts_with("DATA") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            format = parts[1].to_string();
            break;
        }
        line.clear();
    }
    match format.as_str() {
        "ascii" => {
            // ASCII格式处理
            while reader.read_line(&mut line)? > 0 {
                let values: Vec<f32> = line.split_whitespace()
                                           .filter_map(|s| s.parse().ok())
                                           .collect();
                //println!("{:?}",values);
                if values.len() >= 3 {
                    points.push(PointT::new(values[0], values[1],values[2]));
                }else{
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "invalid point data"));
                }
                line.clear();
            }
        }, 
        // "binary" => {
        //     let mut buffer = Vec::new();
        //     reader.read_to_end(&mut buffer)?;
        //     for chunk in buffer.chunks(12) {
        //         if chunk.len() == 12 {
        //             let x = f32::from_le_bytes(chunk[0..4].try_into().unwrap());
        //             let y = f32::from_le_bytes(chunk[4..8].try_into().unwrap());
        //             let z = f32::from_le_bytes(chunk[8..12].try_into().unwrap());
        //             //rintln!("{},{},{}", x, y, z);
        //             points.push(PointT::new(x, y,z));
        //         }
        //     }
        // },
        //TODO: 别的格式支持
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported format")),
    }
    Ok(PointCloud::from_point(points))
}
