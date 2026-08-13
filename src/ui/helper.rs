pub fn ternary<T>(con: bool, on_true: T, on_false: T) -> T {
    if con { on_true } else { on_false }
}
