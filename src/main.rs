use std::fs::File;
use std::io::Write;

fn main() {
    // Replace the default CSG object (a cube) with any valid CSG object.
    let csg_object = csgrs::CSG::cube(None); // Example: Replace with csgrs::CSG::sphere(1.0);

    // Generate STL data
    let stl_data = csg_object.to_stl("PartCAD part");

    // Write the STL file
    let filename = "output.stl";
    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(stl_data.as_bytes()).expect("Failed to write STL");

    println!("STL file '{}' created successfully!", filename);
}

