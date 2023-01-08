/// assign the `src` to `dst` if they're not equal, and then return a "dirty" flag.
pub fn assign_if_ne<T: PartialEq>(dst: &mut T, src: T) -> bool {
    if *dst != src {
        *dst = src;
        true
    } else {
        false
    }
}
