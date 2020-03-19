#[cfg(test)]
mod io_test {
    use hyeong::io;

    #[test]
    fn io_read_file_test01() {
        let t = format!(
            "{:?}",
            io::read_file(if cfg!(target_os = "windows") {
                "examples\\hello_world\\hello_world.hyeong"
            } else {
                "examples/hello_world/hello_world.hyeong"
            })[0]
        );
        assert_eq!("type: 0, cnt1: 9, cnt2: 8, area: \"_\"", t);
    }

    #[test]
    fn io_save_file_test01() {
        io::save_to_file(if cfg!(target_os = "windows") {
            "examples\\hello_world\\hello_world.hyeong"
        } else {
            "examples/hello_world/hello_world.hyeong"
        }, "혀어어어어어어어엉........ 핫. 혀엉..... 흑... 하앗... 흐윽... 형.  하앙.혀엉.... 하앙... 흐윽... 항. 항. 형... 하앙. 흐으윽... 형... 흡... 혀엉..하아아앗. 혀엉.. 흡... 흐읍... 형.. 하앗. 하아앙... 형... 하앙... 흐윽...혀어어엉.. 하앙. 항. 형... 하앙. 혀엉.... 하앙. 흑... 항. 형... 흡  하앗. 
        혀엉..... 흑. 흣".to_string());
        let t = format!(
            "{:?}",
            io::read_file(if cfg!(target_os = "windows") {
                "examples\\hello_world\\hello_world.hyeong"
            } else {
                "examples/hello_world/hello_world.hyeong"
            })[0]
        );
        assert_eq!("type: 0, cnt1: 9, cnt2: 8, area: \"_\"", t);
    }
}
