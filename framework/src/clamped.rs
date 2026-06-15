pub trait Clamped: PartialOrd + Clone {
	fn clamped(&self, min: Option<Self>, max: Option<Self>) -> Self;
}

impl<T: PartialOrd + Copy> Clamped for T {
	fn clamped(&self, min: Option<Self>, max: Option<Self>) -> Self {
		if let Some(min_val) = min && *self < min_val {
			return min_val;
		}
		if let Some(max_val) = max && *self > max_val {
			return max_val;
		}
		*self
	}
}