use boring_file::reader::FileReader;
use boring_file::writer::FileWriter;
use boring_parser::parser::extract_dtd_version;
use boring_parser::{
    boring_structs_110::Boring110, boring_structs_210::Boring210, boring_structs_300::Boring300,
    boring_structs_400::Boring400, parser::Parse,
};

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
            let boring = Boring210::parse_from_str(&utf8_str)?;
            FileWriter::write_json(&boring, &args.output_file)?;
        }
        "3.00" => {
            let boring = Boring300::parse_from_str(&utf8_str)?;
            FileWriter::write_json(&boring, &args.output_file)?;
        }
        "4.00" => {
            let boring = Boring400::parse_from_str(&utf8_str)?;
            FileWriter::write_json(&boring, &args.output_file)?;
        }
        _ => panic!("unexpected version"),
    }

    Ok(())
}
