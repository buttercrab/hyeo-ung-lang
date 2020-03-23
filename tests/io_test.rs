#[cfg(test)]
mod io_test {
    use hyeong::util::{io, util};
    use std::fs;
    use std::path::PathBuf;
    use termcolor::{ColorChoice, StandardStream};

    #[test]
    fn io_read_file_test01() {
        let mut s = StandardStream::stdout(ColorChoice::Auto);

        let t = format!(
            "{:?}",
            util::parse_file(
                &mut s,
                &PathBuf::from("examples/hello_world/hello_world.hyeong")
            )
            .unwrap()[0]
        );

        assert_eq!("type: 0, cnt1: 9, cnt2: 8, area: \"_\"", t);
    }

    #[test]
    fn io_save_file_test01() {
        let mut s = StandardStream::stdout(ColorChoice::Auto);

        io::save_to_file(
            &PathBuf::from("examples/hello_world/hello_world_temp.hyeong"),
            String::from(
                "\
혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 
형. 하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 
형... 흡... 혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 
하아앙... 형... 하앙... 흐윽...혀어어엉.. 하앙. 항. 형... 
하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗. 혀엉..... 흑. 흣",
            ),
        )
        .unwrap();

        let t = format!(
            "{:?}",
            util::parse_file(
                &mut s,
                &PathBuf::from("examples/hello_world/hello_world_temp.hyeong")
            )
            .unwrap()[0]
        );

        assert_eq!("type: 0, cnt1: 9, cnt2: 8, area: \"_\"", t);

        fs::remove_file(&PathBuf::from(
            "examples/hello_world/hello_world_temp.hyeong",
        ))
        .unwrap();
    }
}
