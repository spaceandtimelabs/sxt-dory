use crate::messages::{
    FirstReduceChallenge, FirstReduceMessage, FoldScalarsChallenge, ScalarProductMessage,
    SecondReduceChallenge, SecondReduceMessage,
};

/// Trait that defines the structure of the Dory proof.
///
/// A type implementing this trait acts as both the transcript and the proof serializer.
/// This is because these two concepts are closely related, and likely should use the same
/// underlying serialization.
pub trait ProofBuilder {
    /// The $\mathbb{G}_1$ group
    type G1;
    /// The $\mathbb{G}_2$ group
    type G2;
    /// The target group, $\mathbb{G}_T$
    type GT;
    /// The scalar field, $\mathbb{F}$, of the groups
    type Scalar;

    /// Append a [`FirstReduceMessage`] to the proof and transcript and return a [`FirstReduceChallenge`] drawn from the transcript.
    #[must_use]
    fn append_first_reduce_message(
        self,
        message: FirstReduceMessage<Self::G1, Self::G2, Self::GT>,
    ) -> (FirstReduceChallenge<Self::Scalar>, Self);
    /// Append a [`SecondReduceMessage`] to the proof and transcript and return a [`SecondReduceChallenge`] drawn from the transcript.
    #[must_use]
    fn append_second_reduce_message(
        self,
        message: SecondReduceMessage<Self::G1, Self::G2, Self::GT>,
    ) -> (SecondReduceChallenge<Self::Scalar>, Self);
    /// Draw a [`FoldScalarsChallenge`] from the transcript.
    #[must_use]
    fn challenge_fold_scalars(self) -> (FoldScalarsChallenge<Self::Scalar>, Self);
    /// Append a [`ScalarProductMessage`] to the proof and transcript.
    #[must_use]
    fn append_scalar_product_message(
        self,
        message: ScalarProductMessage<Self::G1, Self::G2>,
    ) -> Self;
}
