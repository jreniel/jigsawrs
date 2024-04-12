use jigsawrs::prelude::*;

#[test]
fn test_2d_a() {
    let verbosity = 1;
    // let mut retv = 0;

    // Setup JIGSAW types
    let mut jjig = JigsawJig::new();
    // let mut geom = JigsawGeometry::new();
    // let mut mesh = JigsawMesh::new();

    // // Define geometry: vertices and edges for a unit square
    // let vertices = vec![
    //     Vertex2::new(0.0, 0.0),
    //     Vertex2::new(1.0, 0.0),
    //     Vertex2::new(1.0, 1.0),
    //     Vertex2::new(0.0, 1.0),
    // ];

    // let edges = vec![
    //     Edge2::new(0, 1),
    //     Edge2::new(1, 2),
    //     Edge2::new(2, 3),
    //     Edge2::new(3, 0),
    // ];

    // geom.set_vertices(vertices);
    // geom.set_edges(edges);
    // geom.set_flags(JigsawGeometryFlags::EUCLIDEAN_MESH);

    // // Configure JIGSAW parameters
    jjig.set_verbosity(verbosity);
    jjig.set_hfun_hmax(0.25);
    jjig.set_hfun_scal(JigsawHfunScale::RELATIVE);
    jjig.set_mesh_dims(2);

    // // Build JIGSAW mesh
    // retv = jjig.run(&geom, None, None, &mut mesh).unwrap(); // Assuming `run` returns a Result

    // // Optionally, assert conditions to ensure correctness
    // assert_eq!(retv, 0, "[2d_a] JIGSAW returned error code: {}", retv);

    // Further assertions can be made here to verify the contents of `mesh`
    // This would depend on the specifics of your `JigsawMesh` implementation
    // For example:
    // assert!(mesh.vertices.len() > 0, "Mesh should have vertices");

    // Note: The actual testing logic might differ based on what you're testing
}
