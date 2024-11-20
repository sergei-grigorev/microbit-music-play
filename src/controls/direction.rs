use defmt::Format;

#[derive(Clone, Copy, Debug, PartialEq, Format)]
pub(crate) enum Direction {
    Left,
    Right,
}
