use crate::types::{Point3DTrait, PointCloud};
use k_nearest::EuclideanDistanceSquared;
// const MAX_NEIGHBORS: usize = 16;
// const MAX_DISTANCE: f32 = 1.0;

struct Point3DAdapter;
// struct ManhattenDistance;
impl<Point: Point3DTrait> k_nearest::Adapter<3, f32, Point> for Point3DAdapter {
    fn get(point: &Point, dimension: usize) -> f32 {
        match dimension {
            0 => point.get_x(),
            1 => point.get_y(),
            2 => point.get_z(),
            _ => unreachable!(),
        }
    }
}
// impl<const N: usize> k_nearest::Metric<N, f32> for ManhattenDistance {
// 	fn distance(left: &[f32; N], right: &[f32; N]) -> f32 {
// 		(0..N)
// 			.map(|d| left[d] - right[d])
// 			.map(|v| v.abs())
// 			.fold(0.0, |sum, v| sum + v)
// 	}
// 	fn distance_plane(position: &[f32; N], plane: f32, dimension: usize) -> f32 {
// 		let diff = position[dimension] - plane;
// 		diff.abs()
// 	}
// }
pub struct KdtreeSearch<Point: Point3DTrait> {
    kdtree: Option<
        k_nearest::KDTree<
            3,                        // dimensions
            f32,                      // type of a value for a dimension
            Point,                    // point type
            Point3DAdapter,           // adapter to allow any point type
            EuclideanDistanceSquared, // metric to calculate distances
        >,
    >,
}

impl<Point: Point3DTrait> KdtreeSearch<Point> {
    pub fn new() -> Self {
        Self { kdtree: None }
    }
    pub fn set_input_cloud(&mut self, input: &PointCloud<Point>) {
        self.kdtree = Some(k_nearest::KDTree::new(input.points.as_slice()));
    }
    pub fn search_by_k(&self, query: &Point, k: usize, max_distance: f32) -> Vec<(f32, usize)> {
        let mut neighbors = vec![(0.0f32, 0 as usize); k];
        if self.kdtree.is_none() {
            return neighbors;
        } else {
            self.kdtree
                .as_ref()
                .unwrap()
                .k_nearest(query, &mut neighbors, max_distance);
            //println!("{:?}",neighbors);
            return neighbors;
        }
    }
}
