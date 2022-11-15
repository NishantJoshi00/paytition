use anchor_lang::prelude::*;

#[error_code]
pub enum PetitionError {
     #[msg("Topic or Link size exceeded to limit")]
    StringSizeOverflow,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Something went wrong while number crunching")]
    MathError,
    #[msg("Consent verification failed")]
    ConsentError
}
