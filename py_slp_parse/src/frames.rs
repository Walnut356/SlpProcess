use std::sync::Arc;

use pyo3::prelude::*;
use slp_parse::events::{item_frames::ItemFrames, post_frame::PostFrames, pre_frame::PreFrames};

#[derive(Clone, Debug)]
#[pyclass(name = "Frames", frozen)]
pub struct PyFrames {
    #[pyo3(get)]
    pub pre: PyPre,
    #[pyo3(get)]
    pub post: PyPost,
}

#[derive(Clone, Debug)]
#[pyclass(name = "PreFrames", frozen)]
pub struct PyPre {
    pub frames: Arc<PreFrames>,
}

#[pymethods]
impl PyPre {
    fn print_frame(&self, idx: usize) -> PyResult<String> {
        Ok(format!("{}", self.frames.get_frame(idx)))
    }

    #[getter]
    fn get_frame_index(&self) -> PyResult<Vec<i32>> {
        Ok(self.frames.frame_index.to_vec())
    }
    #[getter]
    fn get_random_seed(&self) -> PyResult<Vec<u32>> {
        Ok(self.frames.random_seed.to_vec())
    }
    #[getter]
    fn get_action_state(&self) -> PyResult<Vec<u16>> {
        Ok(self.frames.action_state.to_vec())
    }
    #[getter]
    fn get_position(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self.frames.position.iter().map(|p| (p.x, p.y)).collect())
    }
    #[getter]
    fn get_orientation(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.orientation.to_vec())
    }
    #[getter]
    fn get_joystick(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self.frames.joystick.iter().map(|p| (p.x, p.y)).collect())
    }
    #[getter]
    fn get_cstick(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self.frames.cstick.iter().map(|p| (p.x, p.y)).collect())
    }
    #[getter]
    fn get_engine_trigger(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.engine_trigger.to_vec())
    }
    #[getter]
    fn get_engine_buttons(&self) -> PyResult<Vec<u32>> {
        Ok(self.frames.engine_buttons.to_vec())
    }
    #[getter]
    fn get_controller_buttons(&self) -> PyResult<Vec<u16>> {
        Ok(self.frames.controller_buttons.to_vec())
    }
    #[getter]
    fn get_controller_l(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.controller_l.to_vec())
    }
    #[getter]
    fn get_controller_r(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.controller_r.to_vec())
    }
    #[getter]
    fn get_percent(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.frames.percent.as_ref().map(|x| x.to_vec()))
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "PostFrames", frozen)]
pub struct PyPost {
    pub frames: Arc<PostFrames>,
}

#[pymethods]
impl PyPost {
    fn print_frame(&self, idx: usize) -> PyResult<String> {
        Ok(format!("{}", self.frames.get_frame(idx)))
    }

    #[getter]
    fn get_frame_index(&self) -> PyResult<Vec<i32>> {
        Ok(self.frames.frame_index.to_vec())
    }

    #[getter]
    fn get_character(&self) -> PyResult<Vec<u8>> {
        Ok(self.frames.character.to_vec())
    }
    #[getter]
    fn get_action_state(&self) -> PyResult<Vec<u16>> {
        Ok(self.frames.action_state.to_vec())
    }
    #[getter]
    fn get_position(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self.frames.position.iter().map(|p| (p.x, p.y)).collect())
    }
    #[getter]
    fn get_orientation(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.orientation.to_vec())
    }
    #[getter]
    fn get_percent(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.percent.to_vec())
    }
    #[getter]
    fn get_shield_health(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.shield_health.to_vec())
    }
    #[getter]
    fn get_last_attack_landed(&self) -> PyResult<Vec<u8>> {
        Ok(self.frames.last_attack_landed.to_vec())
    }
    #[getter]
    fn get_combo_count(&self) -> PyResult<Vec<u8>> {
        Ok(self.frames.combo_count.to_vec())
    }
    #[getter]
    fn get_last_hit_by(&self) -> PyResult<Vec<u8>> {
        Ok(self.frames.last_hit_by.to_vec())
    }
    #[getter]
    fn get_stocks(&self) -> PyResult<Vec<u8>> {
        Ok(self.frames.stocks.to_vec())
    }

    #[getter]
    fn get_state_frame(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.frames.state_frame.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_flags(&self) -> PyResult<Option<Vec<u64>>> {
        Ok(self.frames.flags.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_misc_as(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.frames.misc_as.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_last_ground_id(&self) -> PyResult<Option<Vec<bool>>> {
        Ok(self.frames.is_grounded.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_jumps_remaining(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.frames.jumps_remaining.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_l_cancel(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.frames.l_cancel.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_hurtbox_state(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.frames.hurtbox_state.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_air_velocity(&self) -> PyResult<Option<Vec<(f32, f32)>>> {
        Ok(self
            .frames
            .air_velocity
            .as_ref()
            .map(|x| x.iter().map(|p| (p.x, p.y)).collect()))
    }
    #[getter]
    fn get_knockback(&self) -> PyResult<Option<Vec<(f32, f32)>>> {
        Ok(self
            .frames
            .knockback
            .as_ref()
            .map(|x| x.iter().map(|p| (p.x, p.y)).collect()))
    }
    #[getter]
    fn get_ground_velocity(&self) -> PyResult<Option<Vec<(f32, f32)>>> {
        Ok(self
            .frames
            .ground_velocity
            .as_ref()
            .map(|x| x.iter().map(|p| (p.x, p.y)).collect()))
    }
    #[getter]
    fn get_hitlag_remaining(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.frames.state_frame.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_animation_index(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.frames.state_frame.as_ref().map(|x| x.to_vec()))
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "ItemFrames", frozen)]
pub struct PyItem {
    pub frames: Arc<ItemFrames>,
}

#[pymethods]
impl PyItem {
    #[getter]
    fn get_frame_index(&self) -> PyResult<Vec<i32>> {
        Ok(self.frames.frame_index.to_vec())
    }
    #[getter]
    fn get_item_id(&self) -> PyResult<Vec<u16>> {
        Ok(self.frames.item_id.to_vec())
    }
    #[getter]
    fn get_state(&self) -> PyResult<Vec<u8>> {
        Ok(self.frames.state.to_vec())
    }
    #[getter]
    fn get_orientation(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.orientation.to_vec())
    }
    #[getter]
    fn get_velocity(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self.frames.velocity.iter().map(|p| (p.x, p.y)).collect())
    }
    #[getter]
    fn get_position(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self.frames.position.iter().map(|p| (p.x, p.y)).collect())
    }
    #[getter]
    fn get_damage_taken(&self) -> PyResult<Vec<u16>> {
        Ok(self.frames.damage_taken.to_vec())
    }
    #[getter]
    fn get_expiration_timer(&self) -> PyResult<Vec<f32>> {
        Ok(self.frames.expiration_timer.to_vec())
    }
    #[getter]
    fn get_spawn_id(&self) -> PyResult<Vec<u32>> {
        Ok(self.frames.spawn_id.to_vec())
    }
    #[getter]
    fn get_missile_type(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.frames.missile_type.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_turnip_type(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.frames.turnip_type.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_is_launched(&self) -> PyResult<Option<Vec<bool>>> {
        Ok(self.frames.launched.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_charge_power(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.frames.charge_power.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_owner(&self) -> PyResult<Option<Vec<i8>>> {
        Ok(self.frames.owner.as_ref().map(|x| x.to_vec()))
    }
}
