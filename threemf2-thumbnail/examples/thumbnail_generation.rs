//! Example: Thumbnail Generation
//!
//! This example demonstrates how to generate a thumbnail from a 3MF model
//! and save it to a file.
//!
//! Run with: cargo run --example thumbnail_generation --features thumbnail-generation

use threemf2::core::{model::Model, object::ObjectKind};
use threemf2_thumbnail::{ThumbnailConfig, ThumbnailGenerator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple cube model for demonstration
    let model = create_cube_model();

    // Configure the thumbnail generator
    let config = ThumbnailConfig::new()
        .with_dimensions(256, 256) // 256x256 pixels
        .with_padding(0.1) // 10% padding around the model
        .with_background_color(240, 240, 240, 255) // Light gray background
        .with_mesh_color(100, 149, 237, 255) // Cornflower blue mesh
        .with_camera_angles(45.0, 30.0); // Camera angles in degrees

    // Create the generator and generate the thumbnail
    let generator = ThumbnailGenerator::new(config);
    let thumbnail = generator.generate(&model)?;

    // Save the thumbnail to a file
    std::fs::write("thumbnail_output.png", &thumbnail.data)?;

    println!("Thumbnail generated successfully!");
    println!("Saved to: thumbnail_output.png");
    println!("Format: {:?}", thumbnail.format);
    println!("Size: {} bytes", thumbnail.data.len());

    Ok(())
}

/// Creates a simple cube model for demonstration
fn create_cube_model() -> Model {
    use threemf2::core::build::{Build, Item};
    use threemf2::core::mesh::{Mesh, Triangle, Triangles, Vertex, Vertices};
    use threemf2::core::object::Object;
    use threemf2::core::resources::Resources;
    use threemf2::core::types::{OptionalResourceId, OptionalResourceIndex};

    fn triangle(v1: u32, v2: u32, v3: u32) -> Triangle {
        Triangle {
            v1,
            v2,
            v3,
            p1: OptionalResourceIndex::none(),
            p2: OptionalResourceIndex::none(),
            p3: OptionalResourceIndex::none(),
            pid: OptionalResourceId::none(),
            mmu_segmentation: None,
            custom_seam: None,
            paint_color: None,
            paint_seam: None,
        }
    }

    // Define vertices for a cube
    let vertices = Vertices {
        vertex: vec![
            Vertex::new(-1.0, -1.0, -1.0), // 0
            Vertex::new(1.0, -1.0, -1.0),  // 1
            Vertex::new(1.0, 1.0, -1.0),   // 2
            Vertex::new(-1.0, 1.0, -1.0),  // 3
            Vertex::new(-1.0, -1.0, 1.0),  // 4
            Vertex::new(1.0, -1.0, 1.0),   // 5
            Vertex::new(1.0, 1.0, 1.0),    // 6
            Vertex::new(-1.0, 1.0, 1.0),   // 7
        ],
    };

    // Define triangles for the cube (6 faces * 2 triangles each = 12 triangles)
    let triangles = Triangles {
        triangle: vec![
            // Front face
            triangle(0, 1, 2),
            triangle(0, 2, 3),
            // Back face
            triangle(5, 4, 7),
            triangle(5, 7, 6),
            // Top face
            triangle(3, 2, 6),
            triangle(3, 6, 7),
            // Bottom face
            triangle(4, 5, 1),
            triangle(4, 1, 0),
            // Left face
            triangle(4, 0, 3),
            triangle(4, 3, 7),
            // Right face
            triangle(1, 5, 6),
            triangle(1, 6, 2),
        ],
    };

    let mesh = Mesh {
        vertices,
        triangles,
        trianglesets: None,
        beamlattice: None,
    };

    let object = Object {
        id: 1,
        kind: Some(ObjectKind::Mesh(mesh)),
        name: Some("Cube".to_string()),
        pid: OptionalResourceId::none(),
        pindex: OptionalResourceIndex::none(),
        thumbnail: None,
        partnumber: None,
        uuid: None,
        objecttype: None,
    };

    let resources = Resources {
        object: vec![object],
        basematerials: vec![],
    };

    let build = Build {
        uuid: None,
        item: vec![Item {
            objectid: 1,
            transform: None,
            partnumber: None,
            uuid: None,
            path: None,
        }],
    };

    Model {
        unit: None,
        metadata: vec![],
        resources,
        build,
        recommendedextensions: None,
        requiredextensions: None,
    }
}
