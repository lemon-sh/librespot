use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand_distr::{Distribution, Normal, Triangular, Uniform};
use std::fmt;

use crate::NUM_CHANNELS;

/// Trait for dithering algorithms.
///
/// Dithering lowers digital-to-analog conversion ("requantization") error,
/// linearizing output, lowering distortion and replacing it with a constant,
/// fixed noise level, which is more pleasant to the ear than the distortion.
///
/// Guidance:
///
///  * On S24, S24_3 and S16, the default is to use triangular dithering.
///    Depending on personal preference you may use Gaussian dithering instead;
///    it's not as good objectively, but it may be preferred subjectively if
///    you are looking for a more "analog" sound akin to tape hiss.
///
///  * Advanced users who know that they have a DAC without noise shaping have
///    a third option: high-passed dithering, which is like triangular dithering
///    except that it moves dithering noise up in frequency where it is less
///    audible. Note: 99% of DACs are of delta-sigma design with noise shaping,
///    so unless you have a multibit / R2R DAC, or otherwise know what you are
///    doing, this is not for you.
///
///  * Don't dither or shape noise on S32 or F32. On F32 it's not supported
///    anyway (there are no integer conversions and so no rounding errors) and
///    on S32 the noise level is so far down that it is simply inaudible even
///    after volume normalisation and control.
pub trait Ditherer {
    /// Creates a new instance of this ditherer.
    fn new() -> Self
    where
        Self: Sized;
    /// Returns the name of this dithering algorithm.
    fn name(&self) -> &'static str;
    /// Returns a dithering noise sample.
    fn noise(&mut self) -> f64;
}

impl fmt::Display for dyn Ditherer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

fn create_rng() -> SmallRng {
    SmallRng::from_os_rng()
}

/// Triangular Probability Density Function (TPDF) ditherer.
///
/// Uses a triangular distribution with ±1 LSB peak-to-peak amplitude.
/// This is the recommended default for most use cases.
pub struct TriangularDitherer {
    cached_rng: SmallRng,
    distribution: Triangular<f64>,
}

impl Ditherer for TriangularDitherer {
    fn new() -> Self {
        Self {
            cached_rng: create_rng(),
            // 2 LSB peak-to-peak needed to linearize the response:
            distribution: Triangular::new(-1.0, 1.0, 0.0).unwrap(),
        }
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    #[inline]
    fn noise(&mut self) -> f64 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

impl TriangularDitherer {
    /// Name of this dithering algorithm ("tpdf").
    pub const NAME: &'static str = "tpdf";
}

/// Gaussian Probability Density Function (GPDF) ditherer.
///
/// Uses a Gaussian distribution with σ = 0.6 LSB. Provides a more "analog"
/// sound than triangular dithering but is objectively less optimal.
pub struct GaussianDitherer {
    cached_rng: SmallRng,
    distribution: Normal<f64>,
}

impl Ditherer for GaussianDitherer {
    fn new() -> Self {
        Self {
            cached_rng: create_rng(),
            // For Gaussian to achieve equivalent decorrelation to triangular dithering, it needs
            // 3-4 dB higher amplitude than TPDF's optimal 0.408 LSB. If optimizing:
            // - minimum correlation: σ ≈ 0.58
            // - perceptual equivalence: σ ≈ 0.65
            // - worst-case performance: σ ≈ 0.70
            //
            // σ = 0.6 LSB is a reasonable compromise that balances mathematical theory with
            // empirical performance across various signal types.
            distribution: Normal::new(0.0, 0.6).unwrap(),
        }
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    #[inline]
    fn noise(&mut self) -> f64 {
        self.distribution.sample(&mut self.cached_rng)
    }
}

impl GaussianDitherer {
    /// Name of this dithering algorithm ("gpdf").
    pub const NAME: &'static str = "gpdf";
}

/// High-passed TPDF ditherer.
///
/// Like triangular dithering but shifts noise to higher frequencies where it
/// is less audible. Only use with DACs that do not have noise shaping
/// (i.e. multibit / R2R DACs).
pub struct HighPassDitherer {
    active_channel: usize,
    previous_noises: [f64; NUM_CHANNELS as usize],
    cached_rng: SmallRng,
    distribution: Uniform<f64>,
}

impl Ditherer for HighPassDitherer {
    fn new() -> Self {
        Self {
            active_channel: 0,
            previous_noises: [0.0; NUM_CHANNELS as usize],
            cached_rng: create_rng(),
            // 1 LSB +/- 1 LSB (previous) = 2 LSB
            distribution: Uniform::new_inclusive(-0.5, 0.5)
                .expect("Failed to create uniform distribution"),
        }
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

    #[inline]
    fn noise(&mut self) -> f64 {
        let new_noise = self.distribution.sample(&mut self.cached_rng);
        let high_passed_noise = new_noise - self.previous_noises[self.active_channel];
        self.previous_noises[self.active_channel] = new_noise;
        self.active_channel ^= 1;
        high_passed_noise
    }
}

impl HighPassDitherer {
    /// Name of this dithering algorithm ("tpdf_hp").
    pub const NAME: &'static str = "tpdf_hp";
}

/// Creates a new boxed ditherer instance.
pub fn mk_ditherer<D: Ditherer + 'static>() -> Box<dyn Ditherer> {
    Box::new(D::new())
}

/// Builder function type for creating ditherer instances.
pub type DithererBuilder = fn() -> Box<dyn Ditherer>;

/// Finds a ditherer builder by name.
///
/// Supported names: "tpdf" (triangular), "gpdf" (Gaussian), "tpdf_hp" (high-passed).
/// Returns `None` if the name is not recognized.
pub fn find_ditherer(name: Option<String>) -> Option<DithererBuilder> {
    match name.as_deref() {
        Some(TriangularDitherer::NAME) => Some(mk_ditherer::<TriangularDitherer>),
        Some(GaussianDitherer::NAME) => Some(mk_ditherer::<GaussianDitherer>),
        Some(HighPassDitherer::NAME) => Some(mk_ditherer::<HighPassDitherer>),
        _ => None,
    }
}
