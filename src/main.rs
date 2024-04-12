use ros_pointcloud2::pcl_utils::{PointXYZ,PointXYZI,PointXYZRGB,PointXYZRGBA,PointXYZRGBNormal,PointXYZINormal,PointXYZL,PointXYZRGBL,PointXYZNormal};
pub mod types;
pub mod io;
pub mod visual;
use crate::types::PointCloud;
use crate::io::load_from_pcd;
use crate::visual::CloudViewer;
//use crate::io::io_read;
fn main() {
    let cloud1 = load_from_pcd::<PointXYZ>("/home/yansu/research_workspace/pcL_study/bin/cloud/scan3.pcd").unwrap();
    let cloud2 = load_from_pcd::<PointXYZRGB>("/home/yansu/research_workspace/pcL_study/bin/cloud/line_cloud_0.pcd").unwrap();
    // for p in &cloud1.points{
    //     println!("{:?}",p);
    // }
    let mut cloud_viewer = CloudViewer::new("test");
    cloud_viewer.add_cloud(&cloud1);
    println!("{:?}",cloud1.size());
    cloud_viewer.show();
}

