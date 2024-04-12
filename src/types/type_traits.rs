use ros_pointcloud2::pcl_utils::{PointXYZ,PointXYZI,PointXYZRGB,PointXYZRGBA,PointXYZRGBNormal,PointXYZINormal,PointXYZL,PointXYZRGBL,PointXYZNormal};
use kiss3d::nalgebra::Point3;
pub trait Point3DTrait{
    fn new(x:f32,y:f32,z:f32) -> Self;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_z(&self) -> f32;
}
pub trait ToKiss3D {
    fn to_kiss3d(&self) -> Point3<f32>;
}
pub trait Point3DVisible:Point3DTrait+ToKiss3D{}

impl Point3DVisible for PointXYZ{}
impl Point3DVisible for PointXYZRGB{}

impl ToKiss3D for PointXYZ{
    fn to_kiss3d(&self) -> Point3<f32>{
        Point3::<f32>::from_slice(&[self.get_x(),self.get_y(), self.get_z()])
    }
}
impl ToKiss3D for PointXYZRGB{
    fn to_kiss3d(&self) -> Point3<f32>{
        Point3::<f32>::from_slice(&[self.get_x(),self.get_y(), self.get_z()])
    }
}
impl Point3DTrait for PointXYZRGB{
    fn new(x:f32,y:f32,z:f32) -> Self{
        Self{
            x:x,
            y:y,
            z:z,
            r:0,
            g:0,
            b:0
        }
    }
    fn get_x(&self) -> f32{
        self.x
    }
    fn get_y(&self) -> f32{
        self.y
    }
    fn get_z(&self) -> f32{
        self.z
    }
}
impl Point3DTrait for PointXYZ{
    fn new(x:f32,y:f32,z:f32) -> Self{
        Self{
            x:x,
            y:y,
            z:z
        }
    }
    fn get_x(&self) -> f32{
        self.x
    }
    fn get_y(&self) -> f32{
        self.y
    }
    fn get_z(&self) -> f32{
        self.z
    }
}