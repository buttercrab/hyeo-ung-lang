use hyeong::util::error::Error;

#[test]
fn io_errors_keep_their_kind_and_message_in_the_diagnostic() {
    let source = std::io::Error::new(std::io::ErrorKind::NotFound, "missing source file");
    let converted: Error = source.into();
    assert!(converted.get_msg().contains("NotFound"));
    assert!(converted.get_msg().contains("missing source file"));
    assert!(converted.get_note().is_empty());
}

#[test]
fn invalid_utf8_is_a_diagnostic_instead_of_lost_input() {
    let source = String::from_utf8(vec![0xff]).unwrap_err();
    let converted: Error = source.into();
    assert!(converted.get_msg().contains("FromUtf8Error"));
    assert!(converted.get_msg().contains("255"));
    assert!(converted.get_note().is_empty());
}

#[test]
fn io_conversion_retains_the_original_error_and_its_note() {
    let converted: std::io::Error = Error::new("cannot read source", "check the path").into();
    assert_eq!(converted.kind(), std::io::ErrorKind::Other);
    assert_eq!(converted.to_string(), "cannot read source");
    let original = converted
        .get_ref()
        .unwrap()
        .downcast_ref::<Error>()
        .unwrap();
    assert_eq!(original.get_msg(), "cannot read source");
    assert_eq!(original.get_note(), "check the path");
}
