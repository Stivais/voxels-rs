use std::cmp::PartialEq;
use ultraviolet::{Mat4, Vec3};
use self::CameraMovement::*;

#[derive(PartialEq)]
pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    //
    // pub perspective: Mat4,
}

impl Camera {
    pub fn create(position: Vec3, yaw: f32, pitch: f32) -> Camera {
        let mut camera = Camera {
            position,
            yaw,
            pitch,
            front: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::zero(),
            right: Vec3::zero(),
            world_up: Vec3::unit_y(),
            // perspective: Mat4::identity(),
        };
        camera.update_vectors();
        camera
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.position + self.front, self.up)
    }

    fn update_vectors(&mut self) {
        let direction = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );

        self.front = direction.normalized();
        self.right = self.front.cross(self.world_up).normalized();
        self.up = self.right.cross(self.front).normalized();
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = 50.0 * delta_time;
        if direction == FORWARD {
            self.position += self.front * velocity;
        }
        if direction == BACKWARD {
            self.position -= self.front * velocity;
        }
        if direction == LEFT {
            self.position -= self.right * velocity;
        }
        if direction == RIGHT {
            self.position += self.right * velocity;
        }
        if direction == UP {
            self.position += self.up * velocity
        }
        if direction == DOWN {
            self.position -= self.up * velocity
        }
    }

    pub fn process_mouse(&mut self, yaw_offset: f32, pitch_offset: f32) {
        self.yaw += yaw_offset;
        self.pitch += pitch_offset;

        if self.pitch > 89.0 {
            self.pitch = 89.0
        } else if self.pitch < -89.0 {
            self.pitch = -89.0
        };

        self.update_vectors()
    }
}