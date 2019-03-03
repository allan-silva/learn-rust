mod sound;

#[cfg(test)]
mod tests {
    use super::sound::instrument::chords;

    #[test]
    fn call_absolute() {
        let s = chords::guitar();
        assert_eq!(s, "GAD");
    }

    #[test]
    fn call_relative() {
        let s = super::sound::instrument::chords::guitar();
        assert_eq!(s, "GAD");
    }
}
