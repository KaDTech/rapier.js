use crate::dynamics::RawJointSet;
use crate::math::{RawRotation, RawVector};
use na::Unit;
use rapier::dynamics::{BallJoint, FixedJoint, JointParams, PrismaticJoint, SpringModel};
use rapier::math::Isometry;
#[cfg(feature = "dim2")]
use rapier::math::Rotation;
use wasm_bindgen::prelude::*;
#[cfg(feature = "dim3")]
use {
    na::{Matrix3, Quaternion, UnitQuaternion, Vector3},
    rapier::dynamics::RevoluteJoint,
    rapier::utils::WBasis,
};

#[wasm_bindgen]
pub enum RawJointType {
    Ball,
    Fixed,
    Prismatic,
    Revolute,
}

#[wasm_bindgen]
pub enum RawSpringModel {
    Disabled,
    VelocityBased,
    AccelerationBased,
    ForceBased,
}

#[wasm_bindgen]
impl RawJointSet {
    /// The unique integer identifier of the first rigid-body this joint it attached to.
    pub fn jointBodyHandle1(&self, handle: u32) -> u32 {
        self.map(handle, |j| j.body1.into_raw_parts().0)
    }

    /// The unique integer identifier of the second rigid-body this joint is attached to.
    pub fn jointBodyHandle2(&self, handle: u32) -> u32 {
        self.map(handle, |j| j.body2.into_raw_parts().0)
    }

    /// The type of this joint given as a string.
    pub fn jointType(&self, handle: u32) -> RawJointType {
        self.map(handle, |j| match &j.params {
            JointParams::BallJoint(_) => RawJointType::Ball,
            JointParams::FixedJoint(_) => RawJointType::Fixed,
            JointParams::PrismaticJoint(_) => RawJointType::Prismatic,
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(_) => RawJointType::Revolute,
        })
    }

    /// The rotation quaternion that aligns this joint's first local axis to the `x` axis.
    #[cfg(feature = "dim3")]
    pub fn jointFrameX1(&self, handle: u32) -> RawRotation {
        self.map(handle, |j| {
            let local_axis1 = match &j.params {
                JointParams::BallJoint(_) => return RawRotation::identity(),
                JointParams::FixedJoint(f) => return RawRotation(f.local_frame1.rotation),
                #[cfg(feature = "dim3")]
                JointParams::RevoluteJoint(r) => *r.local_axis1,
                JointParams::PrismaticJoint(p) => *p.local_axis1(),
            };

            // TODO: can't we just do rotation_between?
            let basis1a = local_axis1.orthonormal_basis()[0];
            let basis1b = local_axis1.cross(&basis1a);

            let rotmat1 = na::Rotation3::from_matrix_unchecked(Matrix3::from_columns(&[
                local_axis1,
                basis1a,
                basis1b,
            ]));

            let axisangle1 = rotmat1.scaled_axis();
            RawRotation(UnitQuaternion::new(axisangle1))
        })
    }

    /// The rotation matrix that aligns this joint's second local axis to the `x` axis.
    #[cfg(feature = "dim3")]
    pub fn jointFrameX2(&self, handle: u32) -> RawRotation {
        self.map(handle, |j| {
            let local_axis2 = match &j.params {
                JointParams::BallJoint(_) => return RawRotation::identity(),
                JointParams::FixedJoint(f) => return RawRotation(f.local_frame2.rotation),
                #[cfg(feature = "dim3")]
                JointParams::RevoluteJoint(r) => *r.local_axis2,
                JointParams::PrismaticJoint(p) => *p.local_axis2(),
            };

            // TODO: can't we just do rotation_between?
            let basis2a = local_axis2.orthonormal_basis()[0];
            let basis2b = local_axis2.cross(&basis2a);

            let rotmat2 = na::Rotation3::from_matrix_unchecked(Matrix3::from_columns(&[
                local_axis2,
                basis2a,
                basis2b,
            ]));

            let axisangle2 = rotmat2.scaled_axis();
            RawRotation(UnitQuaternion::new(axisangle2))
        })
    }

    /// The position of the first anchor of this joint.
    ///
    /// The first anchor gives the position of the points application point on the
    /// local frame of the first rigid-body it is attached to.
    pub fn jointAnchor1(&self, handle: u32) -> RawVector {
        self.map(handle, |j| match &j.params {
            JointParams::BallJoint(b) => RawVector(b.local_anchor1.coords),
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(r) => RawVector(r.local_anchor1.coords),
            JointParams::FixedJoint(f) => RawVector(f.local_frame1.translation.vector),
            JointParams::PrismaticJoint(p) => RawVector(p.local_anchor1.coords),
        })
    }

    /// Sets the position of the first local anchor
    #[cfg(feature = "dim3")]
    pub fn setJointAnchor1(
        &mut self,
        handle: u32,
        newPos: &RawVector,
    ) {
        self.map_mut(handle, |j| match &mut j.params {
            JointParams::BallJoint(b) => b.configure_local_anchor1_position(newPos.0.into()),
            JointParams::RevoluteJoint(r) => r.configure_local_anchor1_position(newPos.0.into()),
            JointParams::PrismaticJoint(p) => p.configure_local_anchor1_position(newPos.0.into()),
            JointParams::FixedJoint(_) => {},
        })
    }

    /// Sets the position of the second local anchor
    #[cfg(feature = "dim3")]
    pub fn setJointAnchor2(
        &mut self,
        handle: u32,
        newPos: &RawVector,
    ) {
        self.map_mut(handle, |j| match &mut j.params {
            JointParams::BallJoint(b) => b.configure_local_anchor2_position(newPos.0.into()),
            JointParams::RevoluteJoint(r) => r.configure_local_anchor2_position(newPos.0.into()),
            JointParams::PrismaticJoint(p) => p.configure_local_anchor2_position(newPos.0.into()),
            JointParams::FixedJoint(_) => {},
        })
    }

    /// The position of the second anchor of this joint.
    ///
    /// The second anchor gives the position of the points application point on the
    /// local frame of the second rigid-body it is attached to.
    pub fn jointAnchor2(&self, handle: u32) -> RawVector {
        self.map(handle, |j| match &j.params {
            JointParams::BallJoint(b) => RawVector(b.local_anchor2.coords),
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(r) => RawVector(r.local_anchor2.coords),
            JointParams::FixedJoint(f) => RawVector(f.local_frame2.translation.vector),
            JointParams::PrismaticJoint(p) => RawVector(p.local_anchor2.coords),
        })
    }

    /// The first axis of this joint, if any.
    ///
    /// For joints where an application axis makes sense (e.g. the revolute and prismatic joins),
    /// this returns the application axis on the first rigid-body this joint is attached to, expressed
    /// in the local-space of this first rigid-body.
    pub fn jointAxis1(&self, handle: u32) -> Option<RawVector> {
        self.map(handle, |j| match &j.params {
            JointParams::BallJoint(_) | JointParams::FixedJoint(_) => None,
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(r) => Some(RawVector(*r.local_axis1)),
            JointParams::PrismaticJoint(p) => Some(RawVector(*p.local_axis1())),
        })
    }

    /// The second axis of this joint, if any.
    ///
    /// For joints where an application axis makes sense (e.g. the revolute and prismatic joins),
    /// this returns the application axis on the second rigid-body this joint is attached to, expressed
    /// in the local-space of this second rigid-body.
    pub fn jointAxis2(&self, handle: u32) -> Option<RawVector> {
        self.map(handle, |j| match &j.params {
            JointParams::BallJoint(_) | JointParams::FixedJoint(_) => None,
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(r) => Some(RawVector(*r.local_axis2)),
            JointParams::PrismaticJoint(p) => Some(RawVector(*p.local_axis2())),
        })
    }

    /// Are the limits for this joint enabled?
    pub fn jointLimitsEnabled(&self, handle: u32) -> bool {
        self.map(handle, |j| match &j.params {
            JointParams::PrismaticJoint(p) => p.limits_enabled,
            _ => false,
        })
    }

    /// If this is a prismatic joint, returns its lower limit.
    pub fn jointLimitsMin(&self, handle: u32) -> f32 {
        self.map(handle, |j| match &j.params {
            JointParams::PrismaticJoint(p) => p.limits[0],
            _ => -f32::MAX,
        })
    }

    /// If this is a prismatic joint, returns its upper limit.
    pub fn jointLimitsMax(&self, handle: u32) -> f32 {
        self.map(handle, |j| match &j.params {
            JointParams::PrismaticJoint(p) => p.limits[1],
            _ => f32::MAX,
        })
    }

    pub fn jointConfigureMotorModel(&mut self, handle: u32, model: RawSpringModel) {
        let model = match model {
            RawSpringModel::Disabled => SpringModel::Disabled,
            RawSpringModel::VelocityBased => SpringModel::VelocityBased,
            RawSpringModel::AccelerationBased => SpringModel::AccelerationBased,
            RawSpringModel::ForceBased => SpringModel::ForceBased,
        };

        self.map_mut(handle, |j| match &mut j.params {
            JointParams::PrismaticJoint(j) => j.configure_motor_model(model),
            JointParams::BallJoint(j) => j.configure_motor_model(model),
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(j) => j.configure_motor_model(model),
            JointParams::FixedJoint(_) => {}
        })
    }

    #[cfg(feature = "dim3")]
    pub fn jointConfigureBallMotorVelocity(
        &mut self,
        handle: u32,
        vx: f32,
        vy: f32,
        vz: f32,
        factor: f32,
    ) {
        let targetVel = Vector3::new(vx, vy, vz);

        self.map_mut(handle, |j| match &mut j.params {
            JointParams::BallJoint(j) => j.configure_motor_velocity(targetVel, factor),
            _ => {}
        })
    }

    #[cfg(feature = "dim3")]
    pub fn jointConfigureBallMotorPosition(
        &mut self,
        handle: u32,
        qw: f32,
        qx: f32,
        qy: f32,
        qz: f32,
        stiffness: f32,
        damping: f32,
    ) {
        let quat = Quaternion::new(qw, qx, qy, qz);

        self.map_mut(handle, |j| match &mut j.params {
            JointParams::BallJoint(j) => {
                if let Some(unit_quat) = UnitQuaternion::try_new(quat, 1.0e-5) {
                    j.configure_motor_position(unit_quat, stiffness, damping)
                }
            }
            _ => {}
        })
    }

    #[cfg(feature = "dim3")]
    pub fn jointConfigureBallMotor(
        &mut self,
        handle: u32,
        qw: f32,
        qx: f32,
        qy: f32,
        qz: f32,
        vx: f32,
        vy: f32,
        vz: f32,
        stiffness: f32,
        damping: f32,
    ) {
        let quat = Quaternion::new(qw, qx, qy, qz);
        let vel = Vector3::new(vx, vy, vz);

        self.map_mut(handle, |j| match &mut j.params {
            JointParams::BallJoint(j) => {
                if let Some(unit_quat) = UnitQuaternion::try_new(quat, 1.0e-5) {
                    j.configure_motor(unit_quat, vel, stiffness, damping)
                }
            }
            _ => {}
        })
    }

    pub fn jointConfigureUnitMotorVelocity(&mut self, handle: u32, targetVel: f32, factor: f32) {
        self.map_mut(handle, |j| match &mut j.params {
            JointParams::PrismaticJoint(j) => j.configure_motor_velocity(targetVel, factor),
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(j) => j.configure_motor_velocity(targetVel, factor),
            JointParams::BallJoint(_j) =>
            {
                #[cfg(feature = "dim2")]
                _j.configure_motor_velocity(targetVel, factor)
            }
            JointParams::FixedJoint(_) => {}
        })
    }

    pub fn jointConfigureUnitMotorPosition(
        &mut self,
        handle: u32,
        targetPos: f32,
        stiffness: f32,
        damping: f32,
    ) {
        self.map_mut(handle, |j| match &mut j.params {
            JointParams::PrismaticJoint(j) => {
                j.configure_motor_position(targetPos, stiffness, damping)
            }
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(j) => {
                j.configure_motor_position(targetPos, stiffness, damping)
            }
            JointParams::BallJoint(_j) =>
            {
                #[cfg(feature = "dim2")]
                _j.configure_motor_position(Rotation::new(targetPos), stiffness, damping)
            }
            JointParams::FixedJoint(_) => {}
        })
    }

    pub fn jointConfigureUnitMotor(
        &mut self,
        handle: u32,
        targetPos: f32,
        targetVel: f32,
        stiffness: f32,
        damping: f32,
    ) {
        self.map_mut(handle, |j| match &mut j.params {
            JointParams::PrismaticJoint(j) => {
                j.configure_motor(targetPos, targetVel, stiffness, damping)
            }
            #[cfg(feature = "dim3")]
            JointParams::RevoluteJoint(j) => {
                j.configure_motor(targetPos, targetVel, stiffness, damping)
            }
            JointParams::BallJoint(_j) =>
            {
                #[cfg(feature = "dim2")]
                _j.configure_motor(Rotation::new(targetPos), targetVel, stiffness, damping)
            }
            JointParams::FixedJoint(_) => {}
        })
    }
}

#[wasm_bindgen]
pub struct RawJointParams(pub(crate) JointParams);

#[wasm_bindgen]
impl RawJointParams {
    /// Create a new joint descriptor that builds Ball joints.
    ///
    /// A ball joints allows three relative rotational degrees of freedom
    /// by preventing any relative translation between the anchors of the
    /// two attached rigid-bodies.
    pub fn ball(anchor1: &RawVector, anchor2: &RawVector) -> Self {
        Self(BallJoint::new(anchor1.0.into(), anchor2.0.into()).into())
    }

    /// Creates a new joint descriptor that builds a Prismatic joint.
    ///
    /// A prismatic joint removes all the degrees of freedom between the
    /// affected bodies, except for the translation along one axis.
    ///
    /// Returns `None` if any of the provided axes cannot be normalized.
    #[cfg(feature = "dim2")]
    pub fn prismatic(
        anchor1: &RawVector,
        axis1: &RawVector,
        anchor2: &RawVector,
        axis2: &RawVector,
        limitsEnabled: bool,
        limitsMin: f32,
        limitsMax: f32,
    ) -> Option<RawJointParams> {
        let axis1 = Unit::try_new(axis1.0, 0.0)?;
        let axis2 = Unit::try_new(axis2.0, 0.0)?;
        let mut joint = PrismaticJoint::new(anchor1.0.into(), axis1, anchor2.0.into(), axis2);

        if limitsEnabled {
            joint.limits_enabled = true;
            joint.limits = [limitsMin, limitsMax];
        }

        Some(Self(joint.into()))
    }

    /// Creates a new joint descriptor that builds a Prismatic joint.
    ///
    /// A prismatic joint removes all the degrees of freedom between the
    /// affected bodies, except for the translation along one axis.
    ///
    /// Returns `None` if any of the provided axes cannot be normalized.
    #[cfg(feature = "dim3")]
    pub fn prismatic(
        anchor1: &RawVector,
        axis1: &RawVector,
        tangent1: &RawVector,
        anchor2: &RawVector,
        axis2: &RawVector,
        tangent2: &RawVector,
        limitsEnabled: bool,
        limitsMin: f32,
        limitsMax: f32,
    ) -> Option<RawJointParams> {
        let axis1 = Unit::try_new(axis1.0, 0.0)?;
        let axis2 = Unit::try_new(axis2.0, 0.0)?;
        let mut joint = PrismaticJoint::new(
            anchor1.0.into(),
            axis1,
            tangent1.0,
            anchor2.0.into(),
            axis2,
            tangent2.0,
        );

        if limitsEnabled {
            joint.limits_enabled = true;
            joint.limits = [limitsMin, limitsMax];
        }

        Some(Self(joint.into()))
    }

    /// Creates a new joint descriptor that builds a Fixed joint.
    ///
    /// A fixed joint removes all the degrees of freedom between the affected bodies.
    pub fn fixed(
        anchor1: &RawVector,
        axes1: &RawRotation,
        anchor2: &RawVector,
        axes2: &RawRotation,
    ) -> RawJointParams {
        let pos1 = Isometry::from_parts(anchor1.0.into(), axes1.0);
        let pos2 = Isometry::from_parts(anchor2.0.into(), axes2.0);
        Self(FixedJoint::new(pos1, pos2).into())
    }

    /// Create a new joint descriptor that builds Revolute joints.
    ///
    /// A revolute joint removes all degrees of freedom between the affected
    /// bodies except for the rotation along one axis.
    #[cfg(feature = "dim3")]
    pub fn revolute(
        anchor1: &RawVector,
        axis1: &RawVector,
        anchor2: &RawVector,
        axis2: &RawVector,
    ) -> Option<RawJointParams> {
        let axis1 = Unit::try_new(axis1.0, 0.0)?;
        let axis2 = Unit::try_new(axis2.0, 0.0)?;
        Some(Self(JointParams::RevoluteJoint(RevoluteJoint::new(
            anchor1.0.into(),
            axis1,
            anchor2.0.into(),
            axis2,
        ))))
    }
}
