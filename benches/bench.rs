#![feature(test)]

extern crate test;

use string_contains_bench::*;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    //Benchmarking the worst case scenario -- the target class is not in the class string

    const CLASS_STRING: &str = "class1 class2 class3 class4 class5 class6 class7 class8 class9 class10";
    const TARGET_CLASS: &str = "class11";

    #[bench]
    fn bench_css_class_contains_split(b: &mut Bencher) {
        b.iter(|| css_class_contains_split(CLASS_STRING, TARGET_CLASS));
    }

    #[bench]
    fn bench_css_class_contains_any(b: &mut Bencher) {
        b.iter(|| css_class_contains_any(CLASS_STRING, TARGET_CLASS));
    }

    #[bench]
    fn bench_css_class_contains_substring(b: &mut Bencher) {
        b.iter(|| css_class_contains_substring(CLASS_STRING, TARGET_CLASS));
    }
}