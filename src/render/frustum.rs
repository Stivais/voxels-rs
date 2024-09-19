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

    pub fn from_matrix(m: &Mat4) -> Self {
        let mut frustum = Frustum::new();
        frustum.set(m);
        frustum
    }

    pub fn set(&mut self, m: &Mat4) {

        #[inline]
        fn create_plane(mat: &Mat4, index: usize) -> Vec4 {
            let mut plane = Vec4::new(
                mat.cols[0].w + mat.cols[0].index(index),
                mat.cols[1].w + mat.cols[1].index(index),
                mat.cols[2].w + mat.cols[2].index(index),
                mat.cols[3].w + mat.cols[3].index(index),
            );
            plane *= 1.0 / plane.mag();
            plane
        }

        self.planes[0] = create_plane(m, 0);
        self.planes[1] = create_plane(m, 0);
        self.planes[2] = create_plane(m, 1);
        self.planes[3] = create_plane(m, 1);
        self.planes[4] = create_plane(m, 2);
        self.planes[5] = create_plane(m, 2);
        //
        // // Left plane (x = -1)
        // let nx = Vec4::new(
        //     m.cols[0].w + m.cols[0].index(0),
        //     m.cols[1].w + m.cols[1].x,
        //     m.cols[2].w + m.cols[2].x,
        //     m.cols[3].w + m.cols[3].x,
        // );
        // invl = 1.0 / (nx.x * nx.x + nx.y * nx.y + nx.z * nx.z).sqrt();
        // self.planes[0] = nx * invl;
        //
        // // Right plane (x = 1)
        // let px = Vec4::new(
        //     m.cols[0].w - m.cols[0].x,
        //     m.cols[1].w - m.cols[1].x,
        //     m.cols[2].w - m.cols[2].x,
        //     m.cols[3].w - m.cols[3].x,
        // );
        // invl = 1.0 / (px.x * px.x + px.y * px.y + px.z * px.z).sqrt();
        // self.planes[1] = px * invl;
        //
        // // Bottom plane (y = -1)
        // let ny = Vec4::new(
        //     m.cols[0].w + m.cols[0].y,
        //     m.cols[1].w + m.cols[1].y,
        //     m.cols[2].w + m.cols[2].y,
        //     m.cols[3].w + m.cols[3].y,
        // );
        // invl = 1.0 / (ny.x * ny.x + ny.y * ny.y + ny.z * ny.z).sqrt();
        // self.planes[2] = ny * invl;
        //
        // // Top plane (y = 1)
        // let py = Vec4::new(
        //     m.cols[0].w - m.cols[0].y,
        //     m.cols[1].w - m.cols[1].y,
        //     m.cols[2].w - m.cols[2].y,
        //     m.cols[3].w - m.cols[3].y,
        // );
        // invl = 1.0 / (py.x * py.x + py.y * py.y + py.z * py.z).sqrt();
        // self.planes[3] = py * invl;
        //
        // // Near plane (z = -1)
        // let nz = Vec4::new(
        //     m.cols[0].w + m.cols[0].z,
        //     m.cols[1].w + m.cols[1].z,
        //     m.cols[2].w + m.cols[2].z,
        //     m.cols[3].w + m.cols[3].z,
        // );
        // invl = 1.0 / (nz.x * nz.x + nz.y * nz.y + nz.z * nz.z).sqrt();
        // self.planes[4] = nz * invl;
        //
        // // Far plane (z = 1)
        // let pz = Vec4::new(
        //     m.cols[0].w - m.cols[0].z,
        //     m.cols[1].w - m.cols[1].z,
        //     m.cols[2].w - m.cols[2].z,
        //     m.cols[3].w - m.cols[3].z,
        // );
        // invl = 1.0 / (pz.x * pz.x + pz.y * pz.y + pz.z * pz.z).sqrt();
        // self.planes[5] = pz * invl;
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