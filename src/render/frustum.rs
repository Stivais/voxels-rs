use std::ops::Index;
use ultraviolet::{Mat4, Vec3, Vec4};

pub struct Frustum {
    planes: [Vec4; 6],
}

impl Frustum {
    pub fn new() -> Self {
        Frustum {
            planes: Default::default(),
        }
    }

    pub fn create(m: Mat4) -> Self {
        let mut frustum = Frustum::new();
        frustum.set(m);
        frustum
    }

    pub fn set(&mut self, view_projection: Mat4) {

        #[inline]
        fn create_plane(mat: Mat4, index: usize) -> Vec4 {
            let mut plane = Vec4::new(
                mat.cols[0].w + mat.cols[0].index(index),
                mat.cols[1].w + mat.cols[1].index(index),
                mat.cols[2].w + mat.cols[2].index(index),
                mat.cols[3].w + mat.cols[3].index(index),
            );
            plane *= 1.0 / plane.mag();
            plane
        }

        self.planes[0] = create_plane(view_projection, 0);
        self.planes[1] = create_plane(view_projection, 0);
        self.planes[2] = create_plane(view_projection, 1);
        self.planes[3] = create_plane(view_projection, 1);
        self.planes[4] = create_plane(view_projection, 2);
        self.planes[5] = create_plane(view_projection, 2);
    }

    pub fn test_aabb(&self, min: Vec3, max: Vec3) -> bool {

        #[inline]
        fn in_plane(plane: &Vec4, min: &Vec3, max: &Vec3) -> bool {
            plane.x * (if plane.x < 0.0 { min.x } else { max.x }) + plane.y * (if plane.y < 0.0 { min.y } else { max.y }) + plane.z * (if plane.z < 0.0 { min.z } else { max.z }) >= -plane.w
        }

        in_plane(&self.planes[0], &min, &max) &&
        in_plane(&self.planes[1], &min, &max) &&
        in_plane(&self.planes[2], &min, &max) &&
        in_plane(&self.planes[3], &min, &max) &&
        in_plane(&self.planes[4], &min, &max) &&
        in_plane(&self.planes[5], &min, &max)
    }
}