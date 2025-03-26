use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::fmt::{Debug, Display};
use core::hash::Hash;

use super::{
    AutotuneError,
    input_generator::{InputGenerator, IntoInputGenerator},
    key_generator::{IntoKeyGenerator, KeyGenerator},
};

/// Default checksum for an operation set
#[cfg(autotune_persistent_cache)]
pub fn compute_checksum<In: Clone + Send + 'static, Out: 'static>(
    autotunables: &[Arc<dyn Tunable<Inputs = In, Output = Out>>],
) -> String {
    let mut checksum = String::new();
    autotunables.iter().for_each(|op| {
        checksum += op.name();
    });
    format!("{:x}", md5::compute(checksum))
}

/// Groups operations of the same type for autotune
#[derive(Clone)]
pub struct TunableSet<K: AutotuneKey, Inputs: Send + 'static, Output: 'static> {
    tunables: Vec<Arc<dyn Tunable<Inputs = Inputs, Output = Output>>>,
    key_gen: Arc<dyn KeyGenerator<K, Inputs>>,
    input_gen: Arc<dyn InputGenerator<K, Inputs>>,
    #[allow(clippy::type_complexity)]
    checksum_override: Option<Arc<dyn Fn(&Self) -> String + Send + Sync>>,
}

impl<K: AutotuneKey, Inputs: Clone + Send + 'static, Output: 'static>
    TunableSet<K, Inputs, Output>
{
    /// Create a tunable set from a key generator and an input generator
    pub fn new<KMarker, IMarker>(
        key_gen: impl IntoKeyGenerator<K, Inputs, KMarker>,
        input_gen: impl IntoInputGenerator<K, Inputs, IMarker>,
    ) -> Self {
        Self {
            tunables: Default::default(),
            input_gen: Arc::new(input_gen.into_input_gen()),
            key_gen: Arc::new(key_gen.into_key_gen()),
            checksum_override: None,
        }
    }

    /// Register a tunable with this tunable set
    pub fn with_tunable<Marker>(
        mut self,
        tunable: impl IntoTunable<Inputs, Output, Marker>,
    ) -> Self {
        self.tunables.push(Arc::new(tunable.into_tunable()));
        self
    }

    /// Override the checksum algorithm
    pub fn with_custom_checksum(
        mut self,
        checksum: impl Fn(&Self) -> String + Send + Sync + 'static,
    ) -> Self {
        self.checksum_override = Some(Arc::new(checksum));
        self
    }

    /// All candidate operations for autotuning this operation type
    /// Operations can run on toy tensors of relevant size
    pub fn autotunables(&self) -> Vec<Arc<dyn Tunable<Inputs = Inputs, Output = Output>>> {
        self.tunables.clone()
    }

    /// Returns the operation for the given index, matching the order
    /// returned by autotunables. Operation obtained here runs on original tensors
    /// Nb: The 0 index is used a "good default".
    pub fn fastest(
        &self,
        fastest_index: usize,
    ) -> Arc<dyn Tunable<Inputs = Inputs, Output = Output>> {
        self.tunables[fastest_index].clone()
    }

    /// Compute a checksum that can invalidate outdated cached auto-tune results.
    #[cfg(autotune_persistent_cache)]
    pub fn compute_checksum(&self) -> String {
        if let Some(checksum_override) = &self.checksum_override {
            checksum_override(self)
        } else {
            compute_checksum(&self.tunables)
        }
    }

    /// Generate a key from a set of inputs
    pub fn generate_key(&self, inputs: &Inputs) -> K {
        self.key_gen.generate(inputs)
    }

    /// Generate a set of test inputs from a key and reference inputs
    pub fn generate_inputs(&self, key: &K, inputs: &Inputs) -> Inputs {
        self.input_gen.generate(key, inputs)
    }
}

/// A tunable entry in a tunable set
pub trait Tunable: Send + Sync + 'static {
    /// Inputs to the tunable function
    type Inputs: Clone;
    /// Output from the tunable function
    type Output;

    /// Run a tuneable function
    fn execute(&self, inputs: Self::Inputs) -> Result<Self::Output, AutotuneError>;

    /// The name of the tuneable function
    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }
}

/// Something that can be turned into a [Tunable]
///
/// # Marker
/// The marker generic is used to work around limitations in the trait resolver that causes
/// conflicting implementation errors.
pub trait IntoTunable<In, Out, Marker> {
    /// The output tunable type
    type Tunable: Tunable<Inputs = In, Output = Out>;

    /// Convert to a tunable
    fn into_tunable(self) -> Self::Tunable;
}

/// Dummy marker for [`IntoTunable`] on [`Tunable`]s
#[doc(hidden)]
pub struct IsIdentity;

impl<T: Tunable> IntoTunable<T::Inputs, T::Output, IsIdentity> for T {
    type Tunable = T;

    fn into_tunable(self) -> Self::Tunable {
        self
    }
}

#[cfg(autotune_persistent_cache)]
/// Trait alias with support for persistent caching
pub trait AutotuneKey:
    Clone
    + Debug
    + PartialEq
    + Eq
    + Hash
    + Display
    + serde::Serialize
    + serde::de::DeserializeOwned
    + Send
    + Sync
    + 'static
{
}
#[cfg(not(autotune_persistent_cache))]
/// Trait alias
pub trait AutotuneKey:
    Clone + Debug + PartialEq + Eq + Hash + Display + Send + Sync + 'static
{
}

impl AutotuneKey for String {}
