use std::fs::File;
use std::io::Write;

mod partcad; // Link the `partcad.rs` file as a module

fn main() {
    // Use the CSG object defined in the external file
    let part = partcad::part();

    // Generate STL data
    let stl_data = part.to_stl("PartCAD part");

    // Write the STL file
    let filename = "output.stl";
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(stl_data.as_bytes()).expect("Failed to write STL");

    println!("STL file '{}' created successfully!", filename);
}

