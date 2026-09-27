use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SchedulerType {
    Euler,
    UniPc,
    DpmSolver,
}

#[derive(Debug, Clone)]
pub struct FlowMatchScheduler {
    pub num_steps: u32,
    pub shift: f32,
    pub timesteps: Vec<f32>,
    pub sigmas: Vec<f32>,
}

impl FlowMatchScheduler {
    pub fn new(num_steps: u32, shift: f32) -> Self {
        let mut timesteps = Vec::with_capacity(num_steps as usize);
        let mut sigmas = Vec::with_capacity(num_steps as usize + 1);

        for i in 0..num_steps {
            let t = 1.0 - (i as f32 / num_steps as f32);
            // Apply flow-matching time-shift transformation
            // shifted_t = (shift * t) / (1 + (shift - 1) * t)
            let shifted_t = if (shift - 1.0).abs() > 1e-4 {
                (shift * t) / (1.0 + (shift - 1.0) * t)
            } else {
                t
            };
            timesteps.push(shifted_t * 1000.0);
            sigmas.push(shifted_t);
        }
        sigmas.push(0.0);

        Self {
            num_steps,
            shift,
            timesteps,
            sigmas,
        }
    }

    pub fn step_euler(&self, step_idx: usize, sample: &[f32], model_output: &[f32]) -> Vec<f32> {
        let dt = self.sigmas[step_idx + 1] - self.sigmas[step_idx];
        sample
            .iter()
            .zip(model_output.iter())
            .map(|(&x, &v)| x + v * dt)
            .collect()
    }
}
