fn main() {
    let array: *const i32 = &[42; 100] as *const i32;

    let mut index: usize = 0;

    while index < 5000 {
        index += 1;

        let out = unsafe { *array.wrapping_add(index) };
        println!("{index}: {out}");
        debug_assert_eq!(out, 42);
    }
}
