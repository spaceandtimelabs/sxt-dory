use crate::builder::ProofBuilder;
use crate::state::ProverState;

/// Implementation of the extended Dory-Innerproduct protocol.
///
/// The protocol is outlined in sections 3.3 & 4.3 of the Dory paper.
pub fn inner_product_prove<Builder, State, G1, G2, GT, Scalar, Setup>(
    builder: Builder,
    state: State,
    setup: &Setup,
    num_rounds: usize,
) -> Builder
where
    Builder: ProofBuilder<G1 = G1, G2 = G2, GT = GT, Scalar = Scalar>,
    State: ProverState<G1 = G1, G2 = G2, GT = GT, Scalar = Scalar, Setup = Setup>,
{
    let (builder, state) = (0..num_rounds).fold((builder, state), |(builder, state), _| {
        let (challenge, builder) =
            builder.append_first_reduce_message(state.compute_first_reduce_message(setup));
        let state = state.reduce_combine(setup, challenge);
        let (challenge, builder) =
            builder.append_second_reduce_message(state.compute_second_reduce_message(setup));
        (builder, state.reduce_fold(setup, challenge))
    });
    let (challenge, builder) = builder.challenge_fold_scalars();
    builder.append_scalar_product_message(state.compute_scalar_product_message(setup, challenge))
}
