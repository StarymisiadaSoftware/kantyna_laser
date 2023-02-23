pub fn anyhow_error_to_stdio_error(err: anyhow::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, err)
}
