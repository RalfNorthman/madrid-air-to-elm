// #[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
// #[repr(u8)]
// pub enum Color {
//     RED = 1,
//     GREEN = 2,
//     BLUE = 3,
// }
//
// #[derive(hdf5::H5Type, Clone, PartialEq, Debug)]
// #[repr(C)]
// pub struct Pixel {
//     xy: (i64, i64),
//     color: Color,
// }

fn main() -> hdf5::Result<()> {
    // use self::Color::*;
    // use ndarray::arr1;

    // so that libhdf5 doesn't print errors to stdout
    let _ = hdf5::silence_errors();

    {
        // read

        let file = hdf5::File::open("air-quality-madrid/madrid.h5", "r")?;
        let master = file.group("/master")?;
        let name = master.name();
        let filename = master.filename();
        let len = master.len();
        println!("Name is: {:?}", name);
        println!("Filename is: {:?}", filename);
        println!("There is: {:?} objects in {:?}", len, name);
    }
    Ok(())
}
