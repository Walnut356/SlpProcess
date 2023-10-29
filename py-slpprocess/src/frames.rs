use std::sync::Arc;

use pyo3::prelude::*;
use slpprocess::events::{post_frame::PostFrames, pre_frame::PreFrames};

#[derive(Clone, Debug)]
#[pyclass(name = "Frames", frozen)]
pub struct PyFrames {
    #[pyo3(get)]
    pub pre: PyPre,
    #[pyo3(get)]
    pub post: PyPost,
}

#[derive(Clone, Debug)]
#[pyclass(name = "Pre", frozen)]
pub struct PyPre {
    pub pre: Arc<PreFrames>,
}

#[pymethods]
impl PyPre {
    #[getter]
    fn get_frame_index(&self) -> PyResult<Vec<i32>> {
        Ok(self.pre.frame_index.to_vec())
    }
    #[getter]
    fn get_random_seed(&self) -> PyResult<Vec<u32>> {
        Ok(self.pre.random_seed.to_vec())
    }
    #[getter]
    fn get_action_state(&self) -> PyResult<Vec<u16>> {
        Ok(self.pre.action_state.to_vec())
    }
    #[getter]
    fn get_position(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self
            .pre
            .position
            .iter()
            .map(|p| (p.x, p.y))
            .collect())
    }
    #[getter]
    fn get_orientation(&self) -> PyResult<Vec<f32>> {
        Ok(self.pre.orientation.to_vec())
    }
    #[getter]
    fn get_joystick(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self
            .pre
            .joystick
            .iter()
            .map(|p| (p.x, p.y))
            .collect())
    }
    #[getter]
    fn get_cstick(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self
            .pre
            .cstick
            .iter()
            .map(|p| (p.x, p.y))
            .collect())
    }
    #[getter]
    fn get_engine_trigger(&self) -> PyResult<Vec<f32>> {
        Ok(self.pre.engine_trigger.to_vec())
    }
    #[getter]
    fn get_engine_buttons(&self) -> PyResult<Vec<u32>> {
        Ok(self.pre.engine_buttons.to_vec())
    }
    #[getter]
    fn get_controller_buttons(&self) -> PyResult<Vec<u16>> {
        Ok(self.pre.controller_buttons.to_vec())
    }
    #[getter]
    fn get_controller_l(&self) -> PyResult<Vec<f32>> {
        Ok(self.pre.controller_l.to_vec())
    }
    #[getter]
    fn get_controller_r(&self) -> PyResult<Vec<f32>> {
        Ok(self.pre.controller_r.to_vec())
    }
    #[getter]
    fn get_percent(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.pre.percent.as_ref().map(|x| x.to_vec()))
    }
}

#[derive(Clone, Debug)]
#[pyclass(name = "Post", frozen)]
pub struct PyPost {
    pub post: Arc<PostFrames>,
}

#[pymethods]
impl PyPost {
    #[getter]
    fn get_frame_index(&self) -> PyResult<Vec<i32>> {
        Ok(self.post.frame_index.to_vec())
    }

    #[getter]
    fn get_character(&self) -> PyResult<Vec<u8>> {
        Ok(self.post.character.to_vec())
    }
    #[getter]
    fn get_action_state(&self) -> PyResult<Vec<u16>> {
        Ok(self.post.action_state.to_vec())
    }
    #[getter]
    fn get_position(&self) -> PyResult<Vec<(f32, f32)>> {
        Ok(self
            .post
            .position
            .iter()
            .map(|p| (p.x, p.y))
            .collect())
    }
    #[getter]
    fn get_orientation(&self) -> PyResult<Vec<f32>> {
        Ok(self.post.orientation.to_vec())
    }
    #[getter]
    fn get_percent(&self) -> PyResult<Vec<f32>> {
        Ok(self.post.percent.to_vec())
    }
    #[getter]
    fn get_shield_health(&self) -> PyResult<Vec<f32>> {
        Ok(self.post.shield_health.to_vec())
    }
    #[getter]
    fn get_last_attack_landed(&self) -> PyResult<Vec<u8>> {
        Ok(self.post.last_attack_landed.to_vec())
    }
    #[getter]
    fn get_combo_count(&self) -> PyResult<Vec<u8>> {
        Ok(self.post.combo_count.to_vec())
    }
    #[getter]
    fn get_last_hit_by(&self) -> PyResult<Vec<u8>> {
        Ok(self.post.last_hit_by.to_vec())
    }
    #[getter]
    fn get_stocks(&self) -> PyResult<Vec<u8>> {
        Ok(self.post.stocks.to_vec())
    }

    #[getter]
    fn get_state_frame(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.post.state_frame.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_flags(&self) -> PyResult<Option<Vec<u64>>> {
        Ok(self.post.flags.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_misc_as(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.post.misc_as.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_last_ground_id(&self) -> PyResult<Option<Vec<bool>>> {
        Ok(self.post.is_grounded.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_jumps_remaining(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.post.jumps_remaining.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_l_cancel(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.post.l_cancel.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_hurtbox_state(&self) -> PyResult<Option<Vec<u8>>> {
        Ok(self.post.hurtbox_state.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_air_velocity(&self) -> PyResult<Option<Vec<(f32, f32)>>> {
        Ok(self
            .post
            .air_velocity
            .as_ref()
            .map(|x| x.iter().map(|p| (p.x, p.y)).collect()))
    }
    #[getter]
    fn get_knockback(&self) -> PyResult<Option<Vec<(f32, f32)>>> {
        Ok(self
            .post
            .knockback
            .as_ref()
            .map(|x| x.iter().map(|p| (p.x, p.y)).collect()))
    }
    #[getter]
    fn get_ground_velocity(&self) -> PyResult<Option<Vec<(f32, f32)>>> {
        Ok(self
            .post
            .ground_velocity
            .as_ref()
            .map(|x| x.iter().map(|p| (p.x, p.y)).collect()))
    }
    #[getter]
    fn get_hitlag_remaining(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.post.state_frame.as_ref().map(|x| x.to_vec()))
    }
    #[getter]
    fn get_animation_index(&self) -> PyResult<Option<Vec<f32>>> {
        Ok(self.post.state_frame.as_ref().map(|x| x.to_vec()))
    }
}
