use crate::internal_prelude::*;
use super::{Mesh, Vertex};



pub fn generate_tetrahedron() -> Mesh
{
	let positions: Vec<Vec3> = 
	vec![
		Vec3::new(0.0, 1.0, 0.0),
		Vec3::new(0.0,         -0.5, 3.0f64.sqrt() / 2.0),
		Vec3::new(3.0 / 4.0, -0.5, -3.0f64.sqrt() / 4.0),
		Vec3::new(-3.0 / 4.0, -0.5, -3.0f64.sqrt() / 4.0)
	];



	let mut vertices: Vec<Vertex> = vec![];

	for i in 0..4
	{
		vertices.push(Vertex::new(positions[i].into(), positions[i].normalize()));
	}
	
	let indices: Vec<[u32;3]> = 
	vec![
		[0, 1, 2], 
		[0, 3, 1],
		[0, 2, 3],
		[1, 3, 2]
    ];

	Mesh::try_new(vertices, indices).unwrap()
}