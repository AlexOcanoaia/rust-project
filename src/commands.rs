use crate::{arguments::{DecodeArguments, EncodeArguments, PrintArguments, RemoveArguments}, png::Png};

pub fn encode(args: EncodeArguments) -> Result<(), ()> {
    todo!()
}

pub fn decode(args: DecodeArguments) -> Result<(), ()> {
    todo!()
}

pub fn print(args: PrintArguments) {
    let tmp_png = Png::from_file(args.file_path);
    let png = tmp_png.unwrap();

    println!("{}", png);
}

pub fn remove(args: RemoveArguments) -> Result<(), ()> {
    todo!()
}

