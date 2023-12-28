use crate::BodyCreationSettings;
use glam::Vec3;
use jolt_sys::{JPC_Activation, JPC_Body, JPC_BodyID, JPC_MotionType, JPC_ObjectLayer};

pub struct BodyInterface(*mut jolt_sys::JPC_BodyInterface);

impl From<*mut jolt_sys::JPC_BodyInterface> for BodyInterface {
    fn from(ptr: *mut jolt_sys::JPC_BodyInterface) -> Self {
        Self(ptr)
    }
}

impl BodyInterface {
    pub fn create_body(&self, body_settings: &BodyCreationSettings) -> Option<*mut JPC_Body> {
        unsafe {
            let body_id = jolt_sys::JPC_BodyInterface_CreateBody(self.0, body_settings.as_jpc());
            if body_id.is_null() {
                None
            } else {
                Some(body_id)
            }
        }
    }

    pub fn create_body_with_id(
        &self,
        body_id: JPC_BodyID,
        body_settings: &BodyCreationSettings,
    ) -> Option<*mut JPC_Body> {
        unsafe {
            let body_id = jolt_sys::JPC_BodyInterface_CreateBodyWithID(
                self.0,
                body_id,
                body_settings.as_jpc(),
            );
            if body_id.is_null() {
                None
            } else {
                Some(body_id)
            }
        }
    }

    pub fn destroy_body(&self, body_id: JPC_BodyID) {
        unsafe {
            jolt_sys::JPC_BodyInterface_DestroyBody(self.0, body_id);
        }
    }

    pub fn add_body(&self, body_id: JPC_BodyID, activation: JPC_Activation) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddBody(self.0, body_id, activation);
        }
    }

    pub fn remove_body(&self, body_id: JPC_BodyID) {
        unsafe {
            jolt_sys::JPC_BodyInterface_RemoveBody(self.0, body_id);
        }
    }

    pub fn create_and_add_body(
        &self,
        body_settings: &BodyCreationSettings,
        activation: JPC_Activation,
    ) -> JPC_BodyID {
        unsafe {
            jolt_sys::JPC_BodyInterface_CreateAndAddBody(self.0, body_settings.as_jpc(), activation)
        }
    }

    pub fn is_added(&self, body_id: JPC_BodyID) -> bool {
        unsafe { jolt_sys::JPC_BodyInterface_IsAdded(self.0, body_id) }
    }

    pub fn set_linear_and_angular_velocity(
        &self,
        body_id: JPC_BodyID,
        linear: Vec3,
        angular: Vec3,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetLinearAndAngularVelocity(
                self.0,
                body_id,
                linear.as_ref().as_ptr(),
                angular.as_ref().as_ptr(),
            );
        }
    }

    pub fn linear_and_angular_velocity(&self, body_id: JPC_BodyID) -> (Vec3, Vec3) {
        unsafe {
            let mut linear = Vec3::ZERO;
            let mut angular = Vec3::ZERO;
            jolt_sys::JPC_BodyInterface_GetLinearAndAngularVelocity(
                self.0,
                body_id,
                linear.as_mut().as_mut_ptr(),
                angular.as_mut().as_mut_ptr(),
            );
            (linear, angular)
        }
    }

    pub fn set_linear_velocity(&self, body_id: JPC_BodyID, velocity: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetLinearVelocity(
                self.0,
                body_id,
                velocity.as_ref().as_ptr(),
            );
        }
    }

    pub fn linear_velocity(&self, body_id: JPC_BodyID) -> Vec3 {
        unsafe {
            let mut result = Vec3::ZERO;
            jolt_sys::JPC_BodyInterface_GetLinearVelocity(
                self.0,
                body_id,
                result.as_mut().as_mut_ptr(),
            );
            result
        }
    }

    pub fn add_linear_velocity(&self, body_id: JPC_BodyID, velocity: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddLinearVelocity(
                self.0,
                body_id,
                velocity.as_ref().as_ptr(),
            );
        }
    }

    pub fn add_linear_and_angular_velocity(
        &self,
        body_id: JPC_BodyID,
        linear: Vec3,
        angular: Vec3,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddLinearAndAngularVelocity(
                self.0,
                body_id,
                linear.as_ref().as_ptr(),
                angular.as_ref().as_ptr(),
            );
        }
    }

    pub fn set_angular_velocity(&self, body_id: JPC_BodyID, velocity: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetAngularVelocity(
                self.0,
                body_id,
                velocity.as_ref().as_ptr(),
            );
        }
    }

    pub fn angular_velocity(&self, body_id: JPC_BodyID) -> Vec3 {
        unsafe {
            let mut result = Vec3::ZERO;
            jolt_sys::JPC_BodyInterface_GetAngularVelocity(
                self.0,
                body_id,
                result.as_mut().as_mut_ptr(),
            );
            result
        }
    }

    pub fn get_point_velocity(&self, body_id: JPC_BodyID, point: Vec3) -> Vec3 {
        unsafe {
            let mut result = Vec3::ZERO;
            jolt_sys::JPC_BodyInterface_GetPointVelocity(
                self.0,
                body_id,
                point.as_ref().as_ptr(),
                result.as_mut().as_mut_ptr(),
            );
            result
        }
    }

    pub fn position(&self, body_id: JPC_BodyID) -> Vec3 {
        unsafe {
            let mut result = Vec3::ZERO;
            jolt_sys::JPC_BodyInterface_GetPosition(self.0, body_id, result.as_mut().as_mut_ptr());
            result
        }
    }

    pub fn set_position(&self, body_id: JPC_BodyID, position: Vec3, activation: JPC_Activation) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetPosition(
                self.0,
                body_id,
                position.as_ref().as_ptr(),
                activation,
            );
        }
    }

    pub fn center_of_mass_position(&self, body_id: JPC_BodyID) -> Vec3 {
        unsafe {
            let mut result = Vec3::ZERO;
            jolt_sys::JPC_BodyInterface_GetCenterOfMassPosition(
                self.0,
                body_id,
                result.as_mut().as_mut_ptr(),
            );
            result
        }
    }

    pub fn rotation(&self, body_id: JPC_BodyID) -> glam::Quat {
        unsafe {
            let mut result = glam::Quat::IDENTITY;
            jolt_sys::JPC_BodyInterface_GetRotation(
                self.0,
                body_id,
                (&mut result) as *mut _ as *mut _,
            );
            result
        }
    }

    pub fn set_rotation(
        &self,
        body_id: JPC_BodyID,
        rotation: glam::Quat,
        activation: JPC_Activation,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetRotation(
                self.0,
                body_id,
                rotation.as_ref().as_ptr(),
                activation,
            );
        }
    }

    pub fn set_position_rotation_and_velocity(
        &self,
        body_id: JPC_BodyID,
        position: Vec3,
        rotation: glam::Quat,
        linear_velocity: Vec3,
        angular_velocity: Vec3,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetPositionRotationAndVelocity(
                self.0,
                body_id,
                position.as_ref().as_ptr(),
                rotation.as_ref().as_ptr(),
                linear_velocity.as_ref().as_ptr(),
                angular_velocity.as_ref().as_ptr(),
            );
        }
    }

    pub fn activate_body(&self, body_id: JPC_BodyID) {
        unsafe {
            jolt_sys::JPC_BodyInterface_ActivateBody(self.0, body_id);
        }
    }

    pub fn deactivate_body(&self, body_id: JPC_BodyID) {
        unsafe {
            jolt_sys::JPC_BodyInterface_DeactivateBody(self.0, body_id);
        }
    }

    pub fn is_active(&self, body_id: JPC_BodyID) -> bool {
        unsafe { jolt_sys::JPC_BodyInterface_IsActive(self.0, body_id) }
    }

    pub fn add_force(&self, body_id: JPC_BodyID, force: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddForce(self.0, body_id, force.as_ref().as_ptr());
        }
    }

    pub fn add_force_at_position(&self, body_id: JPC_BodyID, force: Vec3, position: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddForceAtPosition(
                self.0,
                body_id,
                force.as_ref().as_ptr(),
                position.as_ref().as_ptr(),
            );
        }
    }

    pub fn add_torque(&self, body_id: JPC_BodyID, torque: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddTorque(self.0, body_id, torque.as_ref().as_ptr());
        }
    }

    pub fn add_force_and_torque(&self, body_id: JPC_BodyID, force: Vec3, torque: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddForceAndTorque(
                self.0,
                body_id,
                force.as_ref().as_ptr(),
                torque.as_ref().as_ptr(),
            );
        }
    }

    pub fn add_impulse(&self, body_id: JPC_BodyID, impulse: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddImpulse(self.0, body_id, impulse.as_ref().as_ptr());
        }
    }

    pub fn add_impulse_at_position(&self, body_id: JPC_BodyID, impulse: Vec3, position: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddImpulseAtPosition(
                self.0,
                body_id,
                impulse.as_ref().as_ptr(),
                position.as_ref().as_ptr(),
            );
        }
    }

    pub fn add_angular_impulse(&self, body_id: JPC_BodyID, impulse: Vec3) {
        unsafe {
            jolt_sys::JPC_BodyInterface_AddAngularImpulse(
                self.0,
                body_id,
                impulse.as_ref().as_ptr(),
            );
        }
    }

    pub fn set_motion_type(
        &self,
        body_id: JPC_BodyID,
        motion_type: JPC_MotionType,
        activation: JPC_Activation,
    ) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetMotionType(self.0, body_id, motion_type, activation);
        }
    }

    pub fn motion_type(&self, body_id: JPC_BodyID) -> JPC_MotionType {
        unsafe { jolt_sys::JPC_BodyInterface_GetMotionType(self.0, body_id) }
    }

    pub fn set_object_layer(&self, body_id: JPC_BodyID, layer: JPC_ObjectLayer) {
        unsafe {
            jolt_sys::JPC_BodyInterface_SetObjectLayer(self.0, body_id, layer);
        }
    }

    pub fn object_layer(&self, body_id: JPC_BodyID) -> JPC_ObjectLayer {
        unsafe { jolt_sys::JPC_BodyInterface_GetObjectLayer(self.0, body_id) }
    }
}