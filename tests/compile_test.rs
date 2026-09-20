#[cfg(test)]
mod compile_test {
    use hyeong::core::state::{State, UnOptState};
    use hyeong::core::{compile, parse};
    use hyeong::number::num::Num;
    use hyeong::util::error::Error;

    #[test]
    fn build_source_emits_pending_output() {
        let mut state = UnOptState::new();
        state.get_stack(1).push(Num::from_num(65));
        state.get_stack(2).push(Num::from_num(66));

        let source = compile::build_source(state, &[], 0);

        assert!(source.contains("print!(\"A\");"));
        assert!(source.contains("eprint!(\"B\");"));
    }

    #[test]
    fn build_source_emits_points_on_optimized_level() {
        let code = parse::parse("형..".to_string());
        let mut state = UnOptState::new();
        state.set_point(7, 0);

        let source = compile::build_source(state, &code, 2);

        assert!(source.contains("point.insert(7u128, "));
    }

    #[test]
    fn error_converts_into_io_error() {
        let err = Error::new("message", "note");
        let io_err = std::io::Error::from(err);

        assert_eq!(io_err.to_string(), "message");
    }
}
