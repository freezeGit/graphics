

#[derive(Debug, Default)]
pub struct WPos {
    x: f64,
    y: f64,
}

#[derive(Debug, Default)]
pub struct WVel{
    xv: f64,
    yv: f64,
}
#[derive(Debug)]
pub(crate) struct Body {
    position: WPos,
    velocity: WVel,
}