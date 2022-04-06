#[cfg(test)]
mod tests {
    use discrimenum::{Hash, PartialEq};

    #[test]
    fn should_hash() {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        #[derive(Hash)]
        enum HashTest {
            A(usize),
            B(usize),
        }

        let a1 = HashTest::A(123);
        let a2 = HashTest::A(321);
        let b = HashTest::B(123);

        let hasher_builder = RandomState::new();
        let mut a1_hasher = hasher_builder.build_hasher();
        let mut a2_hasher = hasher_builder.build_hasher();
        let mut b_hasher = hasher_builder.build_hasher();

        {
            use std::hash::Hash;

            a1.hash(&mut a1_hasher);
            a2.hash(&mut a2_hasher);
            b.hash(&mut b_hasher);
        }

        assert_eq!(a1_hasher.finish(), a2_hasher.finish());
        assert_ne!(a1_hasher.finish(), b_hasher.finish());
    }

    #[test]
    fn should_hash_generic() {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        #[derive(Hash)]
        enum HashTest<T> {
            A(T),
            B(T),
        }

        let a1 = HashTest::A(123);
        let a2 = HashTest::A(321);
        let b = HashTest::B(123);

        let hasher_builder = RandomState::new();
        let mut a1_hasher = hasher_builder.build_hasher();
        let mut a2_hasher = hasher_builder.build_hasher();
        let mut b_hasher = hasher_builder.build_hasher();

        {
            use std::hash::Hash;

            a1.hash(&mut a1_hasher);
            a2.hash(&mut a2_hasher);
            b.hash(&mut b_hasher);
        }

        assert_eq!(a1_hasher.finish(), a2_hasher.finish());
        assert_ne!(a1_hasher.finish(), b_hasher.finish());
    }

    #[test]
    fn should_partial_eq() {
        #[derive(Debug, PartialEq)]
        enum PartialEqTest {
            A(usize),
            B(usize),
        }

        let a1 = PartialEqTest::A(123);
        let a2 = PartialEqTest::A(321);
        let b = PartialEqTest::B(123);

        assert_eq!(a1, a2);
        assert_ne!(a1, b);
    }

    #[test]
    fn should_partial_eq_generic() {
        #[derive(Debug, PartialEq)]
        enum PartialEqTest<T: std::fmt::Debug> {
            A(T),
            B(T),
        }

        let a1 = PartialEqTest::A(123);
        let a2 = PartialEqTest::A(321);
        let b = PartialEqTest::B(123);

        assert_eq!(a1, a2);
        assert_ne!(a1, b);
    }
}
