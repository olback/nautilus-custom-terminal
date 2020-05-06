use percent_encoding;

pub fn fix_filename(filename: String) -> String {

    percent_encoding::percent_decode_str(&filename)
        .decode_utf8_lossy()
        .to_string()
        .replace("file://", "")

}
