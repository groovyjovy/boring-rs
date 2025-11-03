use file::reader::FileReader;
use file::writer::FileWriter;
use parser::parser::extract_dtd_version;
use parser::{boring_structs_110::Boring110, parser::Parse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = FileReader::from_args();

    // boring xml is wrote Shift_JIS.
    // convert utf-8 for uril.
    let shift_jis_bytes = FileReader::read_xml(&args)?;
    let utf8_str = String::from_utf8(FileReader::shift_jis_to_utf8(&shift_jis_bytes)?)?;

    let input_version = extract_dtd_version(&utf8_str)?;

    match input_version.as_ref() {
        "1.10" => {
            let boring_110 = Boring110::parse_from_str(&utf8_str)?;
            FileWriter::write_json(&boring_110, &args.output_file)?;
        }
        "2.10" => {
            unimplemented!("this version is not supported.")
        }
        "3.00" => {
            unimplemented!("this version is not supported.")
        }
        "4.00" => {
            unimplemented!("this version is not supported.")
        }
        _ => panic!("unexpected version"),
    }

    Ok(())
}
