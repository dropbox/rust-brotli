use std::io::prelude::*;

fn generate_man_page() -> String {
    man::prelude::Manual::new("brotli")
        .about("A brotli compressor and decompressor.")
        .author(man::prelude::Author::new("Daniel Reiter Horn").email("danielrh@dropbox.com"))
        .flag(man::prelude::Flag::new()
                  .short("-catable")
                  .long("--catable")
                  .help("Enable concatable mode, so that the generated data can be added to block generated with --apendable"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-nothreadpool")
                  .long("--nothreadpool")
                  .help("Disables the work thread pool"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-appendable")
                  .long("--appendable")
                  .help("Enables apendable mode, so that the datastream can be extended with more data from the --catable mode"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-magic")
                  .long("--magic")
                  .help("Enables magic number as the first bytes, so that a brotli compressed stream can be easily detected"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-customdictionary=[FILE]")
                  .help("Reads a custom dictionare from file"),
        )
        .flag(man::prelude::Flag::new()
                  .long("--dump-dictionary")
                  .help("Prints the dictionary used"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-utf8")
                  .help("Forces the utf8 context mode in the compression"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-msb")
                  .help("Forces the msb context mode in the compression"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-lsb")
                  .help("Forces the lsb context mode in the compression"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-signed")
                  .help("Forces the signed context mode in the compression"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-efficient")
                  .help("favor cpu efficiency over low latency"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-lowlatency")
                  .help("favor cpu low latency over efficiency"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-i")
                  .help("display the intermediate representation of metablocks"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-0")
                  .long("-q0")
                  .help("Sets quality = 0"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-1")
                  .long("-q1")
                  .help("Sets quality = 1"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-2")
                  .long("-q2")
                  .help("Sets quality = 2"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-3")
                  .long("-q3")
                  .help("Sets quality = 3"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-4")
                  .long("-q4")
                  .help("Sets quality = 4"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-5")
                  .long("-q5")
                  .help("Sets quality = 5"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-6")
                  .long("-q6")
                  .help("Sets quality = 6"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-7")
                  .long("-q7")
                  .help("Sets quality = 7"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-8")
                  .long("-q8")
                  .help("Sets quality = 8"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-9")
                  .long("-q9")
                  .help("Sets quality = 9"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-10")
                  .long("-q10")
                  .help("Sets quality = 10"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-11")
                  .long("-q11")
                  .help("Sets quality = 11"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-9.5")
                  .long("-q9.5")
                  .help("Sets quality = 10"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-9.5x")
                  .long("-q9.5x")
                  .help("Sets quality = 11"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-q9.5y")
                  .help("Sets quality = 12"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-l[NUMBER]")
                  .help("Sets the input block size"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-j[NUMBER]")
                  .help("Sets maximum size of the threadpool"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-bytescore=[NUMBER]")
                  .help("Sets the literal_byte_score value of the hasher, default 0"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-w[NUMBER]")
                  .help("Sets the lgwin value, default 22"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-validate")
                  .long("--validate")
                  .help("Enables validation"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-bs[NUMBER]")
                  .help("Sets the buffer size"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-findprior")
                  .help("Enables prior bitmask detection"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-findspeed=[NUMBER]")
                  .help("Sets cdf adaptation detection to [NUMBER]"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-basicstride")
                  .help("Sets stride"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-stride")
                  .help("Sets stride"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-advstride")
                  .help("Sets stride"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-speed=[NUMBER]")
                  .help("Sets speed"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-avoiddistanceprefixsearch")
                  .help("Avoids distance prefix search"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-b[NUMBER]")
                  .help("Does benchmark"),
        )
        .flag(man::prelude::Flag::new()
                  .long("-c")
                  .help("Enables compression mode"),
        )
        .flag(man::prelude::Flag::new()
                  .short("-h")
                  .long("-help")
                  .long("--help")
                  .help("Prints the help text"),
        )
        .description("A brotli compressor and decompressor that with an interface avoiding the rust stdlib.
This makes it suitable for embedded devices and kernels. It is designed with a pluggable allocator so that
the standard lib's allocator may be employed. All included code is safe.")
        .example(man::Example::new()
            .command("brotli -c /tmp/input /tmp/output")
            .text("compresses the content of /tmp/input and writes it to /tmp/output")
        )

        .render()
}

fn generate_man_page_file() {
    let mut dest_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    dest_path.push("man-page");

    let res = std::fs::create_dir(&dest_path);
    if let Err(err) = res {
        eprintln!("error: {:?}", err);
    }
    dest_path.push("brotli");

    let res = std::fs::create_dir(&dest_path);
    if let Err(err) = res {
        eprintln!("error: {:?}", err);
    }

    dest_path.push("brotli.1");

    match std::fs::File::create(dest_path) {
        Ok(mut file) => {
            let res = file.write_all(generate_man_page().as_bytes());
            if let Err(err) = res {
                eprintln!("error: {:?}", err);
            }
        },
        Err(err) => {
            eprintln!("error: {:?}", err);
        }
    }
}

fn main() {
    generate_man_page_file();
}
