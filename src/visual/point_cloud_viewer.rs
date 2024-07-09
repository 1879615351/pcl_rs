use crate::types::Point3DVisible;
use crate::types::PointCloud;
use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::nalgebra::Point3;
use kiss3d::window::Window;
pub struct Scene {
    camera_3d: ArcBall,
}
impl Scene {
    pub fn new() -> Self {
        Self {
            camera_3d: ArcBall::new(Point3::new(100.0, 100.0, 100.0), Point3::origin()),
        }
    }
}
pub struct CloudViewer {
    clouds: Vec<Vec<Point3<f32>>>,
    colors: Vec<Vec<Point3<f32>>>,
    window: Window,
    scene: Scene,
}

impl CloudViewer {
    pub fn new(title_name: &str) -> Self {
        let mut viewer = Self {
            clouds: Vec::new(),
            colors: Vec::new(),
            window: Window::new_with_size(title_name, 1024, 768),
            scene: Scene::new(),
        };
        viewer.window.set_light(Light::StickToCamera);
        viewer.window.set_background_color(1.0, 1.0, 1.0);
        viewer.window.set_point_size(1.0);
        viewer.window.add_cube(10.0, 10.0, 10.0);
        viewer
    }
    pub fn add_cloud(&mut self, cloud: &PointCloud<impl Point3DVisible>) {
        let mut kiss3d_cloud = Vec::<Point3<f32>>::new();
        let mut kiss3d_color = Vec::<Point3<f32>>::new();
        for p in cloud.points.iter() {
            let p_kiss3d = p.to_kiss3d();
            let p_color = Point3::<f32>::new(0.5, 0.5, 0.5);
            kiss3d_cloud.push(p_kiss3d);
            kiss3d_color.push(p_color);
        }
        self.clouds.push(kiss3d_cloud);
        self.colors.push(kiss3d_color);
        // let  current_cloud = self.clouds.last();
        // let  current_color = self.colors.last();
        // if   current_cloud.is_some() && current_color.is_some(){
        //     for (point,color) in current_cloud.unwrap().iter().zip(current_color.unwrap().iter())
        //     {
        //         self.window.draw_point(point, color);
        //         println!("{:?}",point)
        //     }
        // }
    }
    pub fn show(&mut self) {
        while self.window.render_with_camera(&mut self.scene.camera_3d) {
            for (clouds, colors) in self.clouds.iter_mut().zip(self.colors.iter_mut()) {
                for (point, color) in clouds.iter_mut().zip(colors.iter_mut()) {
                    self.window.draw_point(point, color);
                }
            }
        }
    }
}
