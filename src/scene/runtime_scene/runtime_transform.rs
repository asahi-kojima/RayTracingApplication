use crate::internal_prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct RuntimeTransform
{
	transform: Mat4,
	inv_transform: Mat4,
	inv_transpose_transform: Mat4,
}



impl RuntimeTransform
{
	pub fn from_transform(transform: Transform) -> Self
	{
		Self
		{
			transform: transform.transform_matrix(),
			inv_transform: transform.inv_transform_matrix(),
			inv_transpose_transform: transform.inv_transpose_transform_matrix(),
		}
	}

	pub fn transform(&self) -> &Mat4
	{
		&self.transform
	}

	pub fn set_transform(&mut self, transform: Mat4)
	{
		self.transform = transform;
	}
}