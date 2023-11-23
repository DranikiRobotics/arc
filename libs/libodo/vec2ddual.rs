use l2math::Float64;
use libtrig::prelude::*;
use libtrig::Vec2D;

use crate::DualNum;

pub struct Vec2DDual<T: Float> {
    pub x: DualNum<T>,
    pub y: DualNum<T>,
}

impl Vec2DDual<Float64> {
    // pub const fn new(v: Vec2D, n: Int) -> Self {
    //     Self {
    //         x: DualNum::constant(v.x(), n as usize),
    //         y: DualNum::constant(v.y(), n as usize),
    //     }
    // }

    //     operator fun plus(v: Vector2d) = Vector2dDual(x + v.x, y + v.y)
    //     operator fun plus(v: Vector2dDual<Param>) = Vector2dDual(x + v.x, y + v.y)
    //     operator fun minus(v: Vector2dDual<Param>) = Vector2dDual(x - v.x, y - v.y)
    //     operator fun unaryMinus() = Vector2dDual(-x, -y)
    //
    //     operator fun div(z: Double) = Vector2dDual(x / z, y / z)
    //
    //     infix fun dot(v: Vector2dDual<Param>) = x * v.x + y * v.y
    //     fun sqrNorm() = this dot this
    //     fun norm() = sqrNorm().sqrt()
    //
    //     fun bind() = Vector2dDual(x, y)
    //
    //     fun <NewParam> reparam(oldParam: DualNum<NewParam>) =
    //         Vector2dDual(x.reparam(oldParam), y.reparam(oldParam))
    //
    //     fun drop(n: Int) = Vector2dDual(x.drop(n), y.drop(n))
    //     fun value() = Vector2d(x.value(), y.value())
    //
    //     // precondition: this is normalized
    //     fun angleCast() = Rotation2dDual(x, y)
}
