use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Camera {
    gl_camera_pos: [f32; 3],
    gl_camera_front: [f32; 3],
    gl_camera_up: [f32; 3],
    gl_camera_sensitivity: f32,
    gl_camera_yaw: f32,
    gl_camera_pitch: f32,
    first_move: bool,
    center_x: f32,
    center_y: f32,
}

impl Camera {
    pub fn new(
        position: [f32; 3],
        up: [f32; 3],
        sensitivity: f32,
        center_x: f32,
        center_y: f32,
    ) -> Self {
        Self {
            gl_camera_pos: position,
            gl_camera_front: [0.0, 0.0, 0.0],
            gl_camera_up: up,
            gl_camera_sensitivity: sensitivity,
            gl_camera_yaw: -90.0,
            gl_camera_pitch: 0.0,
            first_move: true,
            center_x,
            center_y,
        }
    }

    pub fn process_mouse_movement(&mut self, x_pos: f64, y_pos: f64) {
        let x_pos = x_pos as f32;
        let y_pos = y_pos as f32;

        if self.first_move {
            self.center_x = x_pos;
            self.center_y = y_pos;
            self.first_move = false;
            return;
        }

        let mut offset_x = x_pos + self.center_x;
        let mut offset_y = self.center_y - y_pos;

        self.center_x = x_pos;
        self.center_y = y_pos;

        offset_x *= self.gl_camera_sensitivity;
        offset_y *= self.gl_camera_sensitivity;

        self.gl_camera_yaw += offset_x;
        self.gl_camera_pitch += offset_y;

        self.gl_camera_pitch = self.gl_camera_pitch.clamp(-89.0, 89.0);

        let yaw_rad = Camera::to_radiance(self.gl_camera_yaw);
        let pitch_rad = Camera::to_radiance(self.gl_camera_pitch);

        let direction = [
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        ];

        let length = (direction[0].powi(2) + direction[1].powi(2) + direction[2].powi(2)).sqrt();
        self.gl_camera_front = [
            direction[0] / length,
            direction[1] / length,
            direction[2] / length,
        ];
    }

    pub fn move_forward(&mut self, speed: f32) {
        self.gl_camera_pos[0] += self.gl_camera_pos[0] * speed;
        self.gl_camera_pos[1] += self.gl_camera_pos[1] * speed;
        self.gl_camera_pos[2] += self.gl_camera_pos[2] * speed;
    }

    pub fn move_back(&mut self, speed: f32) {
        self.gl_camera_pos[0] -= self.gl_camera_pos[0] * speed;
        self.gl_camera_pos[1] -= self.gl_camera_pos[1] * speed;
        self.gl_camera_pos[2] -= self.gl_camera_pos[2] * speed;
    }

    pub fn move_right(&mut self, speed: f32) {
        let cross_product = Camera::cross(&self, self.gl_camera_front, self.gl_camera_up);
        let normalized = Camera::normalize(&self, cross_product);
        self.gl_camera_pos[0] += normalized[0] * speed;
        self.gl_camera_pos[1] += normalized[1] * speed;
        self.gl_camera_pos[2] += normalized[2] * speed;
    }

    pub fn move_left(&mut self, speed: f32) {
        let cross_product = Camera::cross(&self, self.gl_camera_front, self.gl_camera_up);
        let normalized = Camera::normalize(&self, cross_product);
        self.gl_camera_pos[0] += normalized[0] * speed;
        self.gl_camera_pos[1] += normalized[1] * speed;
        self.gl_camera_pos[2] += normalized[2] * speed;
    }

    fn cross(&self, v1: [f32; 3], v2: [f32; 3]) -> [f32; 3] {
        [
            v1[1] * v2[2] - v1[2] * v2[1],
            v1[2] * v2[0] - v1[0] * v2[2],
            v1[0] * v2[1] - v1[1] * v2[0],
        ]
    }

    fn normalize(&self, v: [f32; 3]) -> [f32; 3] {
        let length = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
        if length != 0.0 {
            [v[0] / length, v[1] / length, v[2] / length]
        } else {
            [0.0, 0.0, 0.0]
        }
    }

    fn to_radiance(deg: f32) -> f32 {
        deg * (PI / 180.0)
    }
}
