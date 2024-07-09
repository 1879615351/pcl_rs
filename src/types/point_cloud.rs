use super::type_traits::Point3DTrait;

pub struct PointCloud<PointT: Point3DTrait> {
    pub points: Vec<PointT>,
    pub size: usize,
}
impl<PointT: Point3DTrait> PointCloud<PointT> {
    pub fn new() -> Self {
        PointCloud {
            points: Vec::<PointT>::new(),
            size: 0,
        }
    }
    pub fn from_point(input_points: Vec<PointT>) -> Self {
        let len = input_points.len();
        Self {
            points: input_points,
            size: len,
        }
    }
    pub fn size(&self) -> usize {
        self.points.len()
    }
}
