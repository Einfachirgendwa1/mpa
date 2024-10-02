macro_rules! in_debug_builds {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $($tt)*
        }
    };
}

fn main() {
    let array: *const i32 = &[42; 100] as *const i32;
    let mut index: usize = 0;

    while index < 5000 {
        index += 1;

        let out = unsafe { *array.wrapping_add(index) };

        in_debug_builds! {
            println!("{index}: {out}");
            assert_eq!(out, 42);
        }
    }
}
