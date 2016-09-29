//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::marker::PhantomData;

use pixel;
use area::{self, Area};

/// A write-only view into a `Buffer`.
///
/// The `view::Write` is a writable borrowed area within a `Buffer` and it's
/// parametrized over two types, the `Pixel` and `Channel`.
///
/// The same details on those types from `Buffer` hold true for `View`, except
/// it doesn't own any `Data`.
#[derive(PartialEq, Debug)]
pub struct Write<'a, P, C>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
{
	owner: Area,
	area:  Area,

	pixel:   PhantomData<P>,
	channel: PhantomData<C>,
	data:    &'a mut [C],
}

impl<'a, P, C> Write<'a, P, C>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
{
	#[doc(hidden)]
	#[inline]
	pub fn new(data: &mut [C], owner: Area, area: Area) -> Write<P, C> {
		Write {
			owner: owner,
			area:  area,

			pixel:   PhantomData,
			channel: PhantomData,
			data:    data,
		}
	}

	/// Get the area.
	#[inline]
	pub fn area(&self) -> Area {
		self.area
	}

	/// Get the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.area.width
	}

	/// Get the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.area.height
	}

	/// Set the `Pixel` at the given coordinates.
	///
	/// # Panics
	///
	/// Requires that `x < self.width()` and `y < self.height()`, otherwise it
	/// will panic.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, value: &P) {
		if x >= self.area.width || y >= self.area.height {
			panic!("out of bounds");
		}

		let channels = P::channels();
		let index    = channels * ((self.area.y + y) as usize * self.owner.width as usize + (self.area.x + x) as usize);

		value.write(&mut self.data[index .. index + channels]);
	}

	/// Get a write-only view of the given area, refining further from the
	/// current.
	///
	/// # Panics
	///
	/// Requires that `x + width <= self.width()` and `y + height <=
	/// self.height()`, otherwise it will panic.
	#[inline]
	pub fn writable(&mut self, area: area::Builder) -> Write<P, C> {
		let area = area.complete(Area::from(0, 0, self.area.width, self.area.height));

		if area.x + area.width > self.area.width || area.y + area.height > self.area.height {
			panic!("out of bounds");
		}

		Write::new(&mut self.data, self.owner, Area { x: area.x + self.area.x, y: area.y + self.area.y, .. area })
	}

	/// Fill the view with the given pixel.
	///
	/// # Example
	///
	/// ```
	/// use picto::read;
	/// use picto::Area;
	/// use picto::color::Rgb;
	///
	/// let mut image = read::from_path::<Rgb, u8, _>("tests/boat.xyz").unwrap();
	/// let mut view  = image.writable(Area::new().x(10).y(10).width(20).height(30));
	///
	/// // Make a 20x20 pixel area black at offset 10,10.
	/// view.fill(&Rgb::new(0.0, 0.0, 0.0));
	/// ```
	#[inline]
	pub fn fill(&mut self, pixel: &P) {
		for (x, y) in self.area.absolute() {
			self.set(x, y, pixel);
		}
	}
}

impl<'a, P, C> From<&'a mut Write<'a, P, C>> for Write<'a, P, C>
	where P: pixel::Write<C>,
	      C: pixel::Channel,
{
	#[inline]
	fn from(value: &'a mut Write<'a, P, C>) -> Write<'a, P, C> {
		Write::new(value.data, value.owner, value.area)
	}
}

#[cfg(test)]
mod test {
	use buffer::Buffer;
	use color::*;
	use area::Area;

	#[test]
	fn set() {
		let mut image = Buffer::<Rgb, u8, Vec<_>>::new(2, 2);
		image.set(0, 0, &Rgb::new(1.0, 0.0, 1.0));

		assert_eq!(Rgb::new(1.0, 0.0, 1.0),
			image.get(0, 0));
	}

	#[test]
	fn writable() {
		let mut image = Buffer::<Rgb, u8, Vec<_>>::new(50, 50);
		let mut image = image.writable(Area::new().x(10).y(10).width(4).height(4));

		assert_eq!(vec![
			(10, 10), (11, 10), (12, 10), (13, 10),
			(10, 11), (11, 11), (12, 11), (13, 11),
			(10, 12), (11, 12), (12, 12), (13, 12),
			(10, 13), (11, 13), (12, 13), (13, 13),
		], image.area().relative().collect::<Vec<_>>());

		let mut image = image.writable(Area::new().x(1).y(1).width(2).height(2));

		assert_eq!(vec![
			(11, 11), (12, 11),
			(11, 12), (12, 12),
		], image.area().relative().collect::<Vec<_>>());

		let image = image.writable(Area::new().width(2).height(1));

		assert_eq!(vec![
			(11, 11), (12, 11),
		], image.area().relative().collect::<Vec<_>>());
	}

	#[test]
	fn fill() {
		let mut image = Buffer::<Rgb, u8, Vec<_>>::new(50, 50);
		{
			let mut view = image.writable(Area::new().x(10).y(10).width(4).height(4));
			view.fill(&Rgb::new(1.0, 1.0, 1.0));
		}

		assert_eq!(Rgb::new(0.0, 0.0, 0.0),
			image.get(0, 0));

		assert_eq!(Rgb::new(1.0, 1.0, 1.0),
			image.get(10, 10));
	}
}
