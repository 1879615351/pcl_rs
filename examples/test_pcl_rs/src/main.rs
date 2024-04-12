use ros_pointcloud2::pcl_utils::{PointXYZ,PointXYZI,PointXYZRGB,PointXYZRGBA,PointXYZRGBNormal,PointXYZINormal,PointXYZL,PointXYZRGBL,PointXYZNormal};
use pcl_rs::io::load_from_pcd;
use pcl_rs::visual::CloudViewer;
fn main() {
    let cloud1 = load_from_pcd::<PointXYZ>("/home/xxx/scan3.pcd").unwrap();
    let cloud2 = load_from_pcd::<PointXYZRGB>("/home/xxx/line_cloud_0.pcd").unwrap();
    let mut cloud_viewer = CloudViewer::new("test");
    cloud_viewer.add_cloud(&cloud1);
    println!("{:?}",cloud1.size());
    cloud_viewer.show();
}

