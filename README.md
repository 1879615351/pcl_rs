# pcl_rs

A pure rust PCL tool that includes functions such as reading PCD files and displaying point clouds, which can be used in conjunction with ros_pointcloud2

## Features

- **Read PointCloud From PCD File**
  - `src/io/io_read.rs` => `load_from_pcd`
- **Visualize PointCloud**
  - `Implement types::type_traits::Point3DVisible trait for point cloud types`
  - `src/visual/point_cloud_viewer.rs` => `Use Struct CloudViewer`

## PointCloud Visualization

![avatar](./assets/point_cloud.png)

## Usage

```rust
use ros_pointcloud2::pcl_utils::{PointXYZ,PointXYZI,PointXYZRGB,PointXYZRGBA,PointXYZRGBNormal,PointXYZINormal,PointXYZL,PointXYZRGBL,PointXYZNormal};
use pcl_rs::io::load_from_pcd;
use pcl_rs::visual::CloudViewer;
fn main() {
    let cloud1 = load_from_pcd::<PointXYZ>("/home/xxx/scan3.pcd").unwrap();  //(**Only ASCII Format Now**)
    let cloud2 = load_from_pcd::<PointXYZRGB>("/home/xxx/line_cloud_0.pcd").unwrap();
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


```

## Road Map
- [x] Read PointCloud From PCD File(**Only ASCII Format Now**)
- [ ] Read PointCloud From More Format File
- [X] KNN Search
- [ ] ICP 
