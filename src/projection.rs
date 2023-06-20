pub mod vk {
    use crate::mat4::Mat4F32;
    use std::convert::From;

    /// Orthographic projection matrix with reversed depth (1 -> 0), for Vulkan coordinate system.
    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4F32 {
        Mat4F32 {
            a00: 2f32 / (right - left),
            a01: 0f32,
            a02: 0f32,
            a03: -(right + left) / (right - left),

            //
            //
            a10: 0f32,
            a11: 2f32 / (bottom - top),
            a12: 0f32,
            a13: -(top + bottom) / (bottom - top),

            //
            //
            a20: 0f32,
            a21: 0f32,
            a22: 1f32 / (near - far),
            a23: -far / (near - far),

            //
            //
            a30: 0f32,
            a31: 0f32,
            a32: 0f32,
            a33: 1f32,
        }
    }

    /// Orthographic projection matrix with reversed depth (1 -> 0), for Vulkan coordinate system.
    #[rustfmt::skip]
    pub fn ortho_symmetric(right: f32, top: f32, near: f32, far: f32) -> Mat4F32 {
        Mat4F32::from(
        [
            2_f32 / right,
            0_f32,
            0_f32,
            -1_f32,

            0_f32,
            2_f32 / top,
            0_f32,
            -1_f32,

            0_f32,
            0_f32,
            1f32 / (near - far),
            -far / (near - far),

            0_f32,
            0_f32,
            0_f32,
            1_f32,
        ])
    }
}
