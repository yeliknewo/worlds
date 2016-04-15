
use dorp::{IdManager, Renderable, RenderableSolidColor, Mat4};
use dorp::graphics::solid_color::{DrawMethod, DepthTestMethod, CullingMethod};

use core::{WWorld};

pub fn new_base(manager: &mut IdManager, world: &mut WWorld) -> Renderable {
    let mut renderable = Renderable::new();
    let mut solid_color = RenderableSolidColor::new(manager);
    solid_color.set_draw_method(DrawMethod::Both(DepthTestMethod::IfLess, CullingMethod::Clockwise));
    solid_color.set_perspective(Mat4::orthographic(0.01, 100.0, 75.0, world.get_aspect_ratio()));
    solid_color.set_view(Mat4::identity());
    renderable.set_solid_color(solid_color);
    renderable
}
