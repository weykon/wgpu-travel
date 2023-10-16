
pub struct Camera { 
    eye: glam::Vec3,
    target: glam::Vec3,
    up: glam::Vec3,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}
impl Camera {
    fn build_view_projection_matrix(&self) -> glam::Mat4 {
        // 1.
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = glam::Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        // 3.
        return proj * view;
    }
}