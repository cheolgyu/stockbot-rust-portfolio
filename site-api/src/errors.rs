error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
        BoxError(Box<dyn std::error::Error + Send + Sync>);
        SQLXError(sqlx::Error);
        //DecimalError(rust_decimal::Error);
        //ParseError(chrono::format::ParseError);

    }
    errors {
        ServerApiViewBoundErr(t: String) {
            description("invalid toolchain name")
            display("서버 api view bound 오류 : '{}'", t)
        }

        // You can also add commas after description/display.
        // This may work better with some editor auto-indentation modes:
        UnknownToolchainVersion(v: String) {
            description("unknown toolchain version"), // note the ,
            display("unknown toolchain version: '{}'", v), // trailing comma is allowed
        }
    }
}
