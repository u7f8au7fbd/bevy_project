use bevy::{
    core::FrameCount,
    ecs::system::{Query, Res, ResMut},
    gizmos::{gizmos::Gizmos, GizmoConfig},
    math::Vec3,
    render::color::Color,
    window::Window,
};

pub fn enable_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}

pub fn gizmos_xyz(mut gizmos: Gizmos, mut config: ResMut<GizmoConfig>) {
    let grid_scale = 64.0;
    config.depth_bias = -1.0;
    config.line_width = 0.2;

    gizmos.line(
        //X軸
        Vec3::new(grid_scale, 0.0, 0.0),
        Vec3::new(-grid_scale, 0.0, 0.0),
        Color::RED,
    );
    gizmos.line(
        //Y軸
        Vec3::new(0.0, grid_scale, 0.0),
        Vec3::new(0.0, -grid_scale, 0.0),
        Color::GREEN,
    );
    gizmos.line(
        //Z軸
        Vec3::new(0.0, 0.0, grid_scale),
        Vec3::new(0.0, 0.0, -grid_scale),
        Color::BLUE,
    );
}
