// RUSTSEC-2024-0429: the variadic C out-pointer must be mutable.
use glib::prelude::*;
#[test]
fn variant_str_iter_out_pointer_survives_optimized_calls() {
    let variant = ["alpha", "中文验收", "omega"].to_variant();
    for _ in 0..256 {
        let mut iter = variant.array_iter_str().unwrap();
        assert_eq!(iter.next(), Some("alpha"));
        assert_eq!(iter.next_back(), Some("omega"));
        assert_eq!(iter.next(), Some("中文验收"));
        assert_eq!(iter.next(), None);
        assert_eq!(variant.array_iter_str().unwrap().nth(1), Some("中文验收"));
        assert_eq!(variant.array_iter_str().unwrap().nth_back(1), Some("中文验收"));
        assert_eq!(variant.array_iter_str().unwrap().last(), Some("omega"));
    }
}
