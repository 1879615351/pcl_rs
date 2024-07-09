use ros_pointcloud2::points::{PointXYZ,PointXYZI,PointXYZRGB,PointXYZRGBA,PointXYZRGBNormal,PointXYZINormal,PointXYZL,PointXYZRGBL,PointXYZNormal};
use pcl_rs::io::load_from_pcd;
use pcl_rs::visual::CloudViewer;
use pcl_rs::search::KdtreeSearch;
fn main() {
    let cloud1 = load_from_pcd::<PointXYZ>("/home/xxx/projectspace/pcl_rs/pcl_rs/assets/scan3.pcd").unwrap();
    let cloud2 = load_from_pcd::<PointXYZRGB>("/home/xxx/projectspace/pcl_rs/pcl_rs/assets/scan3.pcd").unwrap();
    let mut cloud_viewer = CloudViewer::new("test");
    let mut kd_tree = KdtreeSearch::new();
    kd_tree.set_input_cloud(&cloud1);
    let point = cloud1.points.get(250).unwrap();
    let res = kd_tree.search_by_k(point, 3,1.0);
    for (distance,index) in &res{
        println!("{:?}",distance);
        println!("{:?}",index);
        println!("{:?}",cloud1.points.get(*index).unwrap());
    }
    cloud_viewer.add_cloud(&cloud1);
    println!("{:?}",cloud1.size());
    cloud_viewer.show();
}

