#[cfg(test)]
mod tests {
    use generate_random::GenerateRandom;

    fn rng() -> impl rand::Rng {
        use rand::SeedableRng;
        rand_chacha::ChaCha8Rng::from(rand_chacha::ChaCha8Core::from_seed([37; 32]))
    }

    fn repeat<T>(count: usize, f: impl FnMut() -> T) -> Vec<T> {
        std::iter::repeat_with(f).take(count).collect()
    }

    #[derive(GenerateRandom, PartialEq, Eq, Debug)]
    enum TestEnumUnit {
        Left,
        #[weight(2)]
        Right,
    }

    #[derive(GenerateRandom, PartialEq, Eq, Debug)]
    enum TestEnumUnnamed {
        Left(u8),
        #[weight(2)]
        Right(bool),
    }

    #[derive(GenerateRandom, PartialEq, Eq, Debug)]
    enum TestEnumNamed {
        Left { x: u8 },
        #[weight(2)]
        Right { y: bool },
    }

    #[derive(GenerateRandom, PartialEq, Eq, Debug)]
    struct TestStructUnit;

    #[derive(GenerateRandom, PartialEq, Eq, Debug)]
    struct TestStructUnnamed(u8, i32);

    #[derive(GenerateRandom, PartialEq, Eq, Debug)]
    struct TestStructNamed {
        x: u8,
        y: i32,
    }

    #[test]
    fn test_enum_unit() {
        let mut rng = rng();
        assert_eq!(
            repeat(
                6,
                || TestEnumUnit::generate_random(&mut rng),
            ),
            vec![
                TestEnumUnit::Right,
                TestEnumUnit::Right,
                TestEnumUnit::Left,
                TestEnumUnit::Right,
                TestEnumUnit::Right,
                TestEnumUnit::Right,
            ]);
    }

    #[test]
    fn test_enum_unnamed() {
        let mut rng = rng();
        assert_eq!(
            repeat(
                6,
                || TestEnumUnnamed::generate_random(&mut rng),
            ),
            vec![
                TestEnumUnnamed::Right(false),
                TestEnumUnnamed::Left(142),
                TestEnumUnnamed::Right(false),
                TestEnumUnnamed::Right(true),
                TestEnumUnnamed::Right(false),
                TestEnumUnnamed::Left(19),
            ]);
    }

    #[test]
    fn test_enum_named() {
        let mut rng = rng();
        assert_eq!(
            repeat(
                6,
                || TestEnumNamed::generate_random(&mut rng),
            ),
            vec![
                TestEnumNamed::Right { y: false },
                TestEnumNamed::Left { x: 142 },
                TestEnumNamed::Right { y: false },
                TestEnumNamed::Right { y: true },
                TestEnumNamed::Right { y: false },
                TestEnumNamed::Left { x: 19 },
            ]);
    }

    #[test]
    fn test_struct_unit() {
        let mut rng = rng();
        assert_eq!(
            TestStructUnit::generate_random(&mut rng),
            TestStructUnit
        );
    }

    #[test]
    fn test_struct_unnamed() {
        let mut rng = rng();
        assert_eq!(
            TestStructUnnamed::generate_random(&mut rng),
            TestStructUnnamed(55, -896102575)
        );
    }

    #[test]
    fn test_struct_named() {
        let mut rng = rng();
        assert_eq!(
            TestStructNamed::generate_random(&mut rng),
            TestStructNamed {
                x: 55,
                y: -896102575,
            }
        );
    }
}
