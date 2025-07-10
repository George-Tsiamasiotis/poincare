use crate::Result;
use rgsl::InterpAccel;

pub(crate) struct Accelerator {
    pub(crate) gsl_accel: InterpAccel,
}

impl Accelerator {
    pub(crate) fn build() -> Result<Self> {
        let gsl_accel = InterpAccel::new();
        Ok(Accelerator { gsl_accel })
    }

    pub(crate) fn reset(&mut self) {
        self.gsl_accel.reset();
    }
}

impl std::fmt::Debug for Accelerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Accelerator")
            .field("hits", &self.gsl_accel.0.hit_count)
            .field("misses", &self.gsl_accel.0.miss_count)
            .finish()
    }
}
