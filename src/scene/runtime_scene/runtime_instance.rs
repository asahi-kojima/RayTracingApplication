use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct RuntimeInstanceId(pub(crate) usize);



#[derive(Debug, Clone)]
pub(crate) struct RuntimeInstance
{
	object_name: String,
	primitive_id: RuntimePrimitiveId,
	material_id: MaterialId,
	transform: RuntimeTransform,
	is_visible: bool,
}


impl RuntimeInstance
{
	pub fn new(
		object_name: String,
		primitive_id: RuntimePrimitiveId,
		material_id: MaterialId,
		transform: RuntimeTransform,
		is_visible: bool,
	) -> Self
	{
		Self {
			object_name,
			primitive_id,
			material_id,
			transform,
			is_visible,
		}
	}

	pub fn object_name(&self) -> &str
	{
		&self.object_name
	}

	pub fn primitive_id(&self) -> RuntimePrimitiveId
	{
		self.primitive_id
	}

	pub fn material_id(&self) -> MaterialId
	{
		self.material_id
	}

	pub fn transform(&self) -> &RuntimeTransform
	{
		&self.transform
	}

	pub fn is_visible(&self) -> bool
	{
		self.is_visible
	}

	pub fn set_visible(&mut self, is_visible: bool)
	{
		self.is_visible = is_visible;
	}

	pub fn set_transform(&mut self, transform: RuntimeTransform)
	{
		self.transform = transform;
	}
}