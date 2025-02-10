use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid move: The player cannot exchange carrots on this field.")]
    CanNotExchangeCarrotsHere,
    #[error("Invalid move: The Player has not enough carrots.")]
    NotEnoughCarrots,
    #[error("Invalid move: The Player has no salads.")]
    NoSalads,
    #[error("Invalid move: The Player has too many carrots.")]
    TooManyCarrots,
    #[error("Invalid move: The Player has too many salads.")]
    TooManySalads,
    #[error("Invalid move: The Player tried to eat a salad without standing on a salad field")]
    NoEatingSaladsHere,
    #[error("Invalid move: The Player tried to eat a salad but they ate one last round")]
    CanNotEatSaladsTwiceInARow,
    #[error("Invalid move: The Player dose not own any swap carrots cards.")]
    MissingCardSwapCarrots,
    #[error("Invalid move: The Player dose not own any eat salad cards.")]
    MissingCardEatSalad,
    #[error("Invalid move: The Player dose not own any fall back cards.")]
    MissingCardFallBack,
    #[error("Invalid move: The Player dose not own any hurry ahead cards.")]
    MissingCardHurryAhead,
    #[error("Invalid move: The Player can not use cards here.")]
    CanNotUseCardsHere,
    #[error("Invalid move: The Player dose not own any cards.")]
    NoCardsOwnd,
    #[error("Invalid move: The Player did not buy a card on the Market.")]
    NoCardPurchased,
    #[error("Invalid move: The Player did not play a card on a Hare field.")]
    NoCardPlayd,
    #[error("Invalid move: This Field is already occupied")]
    FieldIsOccupied,
    #[error("Invalid move: There are no free hedgehog field behind the player")]
    NoAvailableHedgehogField,
    #[error("Invalid move: The player tried to step on a hedgehog field while moving forward.")]
    EnterdHedgehogFieldWhileMovingForward,
    #[error("Invalid move: The player left the bord.")]
    OutOfBounce,
    #[error("Invalid move: Can not return to start field.")]
    CanNotReturnToStart,
    #[error("Invalid move: Other")]
    Other
}