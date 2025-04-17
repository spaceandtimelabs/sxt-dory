use crate::messages::{
    FirstReduceChallenge, FirstReduceMessage, FoldScalarsChallenge, ScalarProductMessage,
    SecondReduceChallenge, SecondReduceMessage,
};

/// Trait for the state and computation and state of the Dory protocol.
///
/// A type implementing this trait primarily stores the $v_i$ and $s_i$ vectors.
/// The trait methods define the operations needed to compute the messages exchanged.
/// This trait is not responsible for the actual messaging. That is the job of the
/// [`ProofBuilder`](crate::ProofBuilder) trait.
pub trait ProverState {
    /// The $\mathbb{G}_1$ group
    type G1;
    /// The $\mathbb{G}_2$ group
    type G2;
    /// The target group, $\mathbb{G}_T$
    type GT;
    /// The scalar, $\mathbb{F}$, field of the groups
    type Scalar;
    /// The setup type. This should contain the public parameters needed for the protocol.
    type Setup;

    /// Computes the [`FirstReduceMessage`] from the state.
    /// That is,
    /// $$\begin{aligned}
    /// D_{1L} &= \langle\vec{v_{1L}},\Gamma_2^{\prime}\rangle & D_{1R} &= \langle\vec{v_{1R}},\Gamma_2^{\prime}\rangle \\\\
    /// D_{2L} &= \langle\Gamma_1^{\prime},\vec{v_{2L}}\rangle & D_{2R} &= \langle\Gamma_1^{\prime},\vec{v_{2R}}\rangle \\\\
    /// E_{1\beta} &= \langle\Gamma_1,\vec{s_2}\rangle & E_{2\beta} &= \langle\vec{s_1},\Gamma_2\rangle
    /// \end{aligned}$$
    ///
    /// # Panics
    /// Panics if the state is not in an appropriate round. That is, if the last Reduce round has not been completed. This method
    /// assumes that the $v_i$ and $s_i$ vectors are of length at least 2.
    #[must_use]
    fn compute_first_reduce_message(
        &self,
        setup: &Self::Setup,
    ) -> FirstReduceMessage<Self::G1, Self::G2, Self::GT>;
    /// Combines $\vec{v_i}$ and $\Gamma_i$ using the [`FirstReduceChallenge`].
    /// That is,
    /// $$\begin{aligned}
    /// \vec{v_1} &\leftarrow \vec{v_1} + \beta\Gamma_1 & \vec{v_2} &\leftarrow \vec{v_2} + \beta^{-1}\Gamma_2
    /// \end{aligned}$$
    ///
    /// # Panics
    /// Panics if the state is not in an appropriate round. That is, if the last Reduce round has not been completed. This method
    /// assumes that the $v_i$ and $s_i$ vectors are of length at least 2.
    #[must_use]
    fn reduce_combine(
        self,
        setup: &Self::Setup,
        first_challenge: FirstReduceChallenge<Self::Scalar>,
    ) -> Self;
    /// Computes the [`SecondReduceMessage`] from the state.
    /// That is,
    /// $$\begin{aligned}
    /// C_+ &= \langle\vec{v_{1L}}, \vec{v_{2R}}\rangle & C_- &= \langle\vec{v_{1R}}, \vec{v_{2L}}\rangle \\\\
    /// E_{1+} &= \langle\vec{v_{1L}}, \vec{s_{2R}}\rangle & E_{1-} &= \langle\vec{v_{1R}}, \vec{s_{2L}}\rangle \\\\
    /// E_{2+} &= \langle\vec{s_{1L}}, \vec{v_{2R}}\rangle & E_{2-} &= \langle\vec{s_{1R}}, \vec{v_{2L}}\rangle
    /// \end{aligned}$$
    ///
    /// # Panics
    /// Panics if the state is not in an appropriate round. That is, if the last Reduce round has not been completed. This method
    /// assumes that the $v_i$ and $s_i$ vectors are of length at least 2.
    #[must_use]
    fn compute_second_reduce_message(
        &self,
        setup: &Self::Setup,
    ) -> SecondReduceMessage<Self::G1, Self::G2, Self::GT>;
    /// Folds the $v_i$ and $s_i$ vectors using the [`SecondReduceChallenge`].
    /// That is,
    /// $$\begin{aligned}
    /// \vec{v_1}^\prime &\leftarrow \alpha \vec{v_{1L}} + \vec{v_{1R}} & \vec{v_2}^\prime &\leftarrow \alpha^{-1} \vec{v_{2L}} + \vec{v_{2R}} \\\\
    /// \vec{s_1}^\prime &\leftarrow \alpha \vec{s_{1L}} + \vec{s_{1R}} & \vec{s_2}^\prime &\leftarrow \alpha^{-1} \vec{s_{2L}} + \vec{s_{2R}}
    /// \end{aligned}$$
    ///
    /// # Panics
    /// Panics if the state is not in an appropriate round. That is, if the last Reduce round has not been completed. This method
    /// assumes that the $v_i$ and $s_i$ vectors are of length at least 2.
    #[must_use]
    fn reduce_fold(
        self,
        setup: &Self::Setup,
        second_challenge: SecondReduceChallenge<Self::Scalar>,
    ) -> Self;
    /// Computes the [`ScalarProductMessage`] using [`FoldScalarsChallenge`]. That is,
    /// $$\begin{aligned}
    /// E_1 &= v_1 + \gamma s_1 H_2 & E_2 &= v_2 + \gamma^{-1} s_2 H_1
    /// \end{aligned}$$
    ///
    /// # Panics
    /// Panics if the state is not in the appropriate round. That is, if the last Reduce round has been completed. This method
    /// assumes that the $v_i$ and $s_i$ vectors are of length 1.
    #[must_use]
    fn compute_scalar_product_message(
        self,
        setup: &Self::Setup,
        fold_scalars_challenge: FoldScalarsChallenge<Self::Scalar>,
    ) -> ScalarProductMessage<Self::G1, Self::G2>;
}
