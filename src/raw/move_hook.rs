/// Hook to be executed each time a value changes it's index inside [super::RawTable].
///
/// This allows building values aware of their own location in memory (which can be computed from the address of the
/// [super::RawTableInner]).
/// Gated behind the `move-hook` feature to reduce compile times, if unneeded.
pub trait MoveHook<T> {
    /// Set the index of a moved value. This is also called, when the value is first inserted into [super::RawTable].
    ///
    /// Note that the index is not updated on value removal from the parent [super::RawTable] and should be considered
    /// no longer valid.
    ///
    /// # SAFETY
    ///
    /// This method should be a simple assignment operation and not panic under any circumstances.
    unsafe fn set_index(val: &mut T, i: usize);
}

/// Implementation of [MoveHook] that does nothing and gets optimised out, resulting in zero runtime cost
pub struct NopMoveHook;

impl<T> MoveHook<T> for NopMoveHook {
    #[inline]
    unsafe fn set_index(_: &mut T, _: usize) {}
}
