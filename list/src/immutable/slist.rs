use std::rc::Rc;

pub struct SList<T> {
	head : SLink<T>,
}

pub struct Iter<'a, T> {
	next : Option<&'a SNode<T>>
}

type SLink<T> = Option<Rc<SNode<T>>>;

struct SNode<T> {
	next : SLink<T>,
	elem : T,
}

impl<T> SList<T> {
	pub fn new() -> Self {
		SList { head : None }
	}

	pub fn append(&self, elem: T) -> SList<T> {
		SList { head:
			Some(Rc::new(SNode{
				next: self.head.clone(),
				elem: elem,
		}))}
	}

	pub fn head(&self) -> Option<&T> {
		self.head.as_ref().map(|rc_ref| &(**rc_ref).elem)
	}

	pub fn tail(&self) -> SList<T> {
		SList { head: self.head.as_ref().and_then(|rc_ref| (**rc_ref).next.clone()) }
	}

	pub fn iter(&self) -> Iter<'_, T> {
		Iter { next: self.head.as_deref() }
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_deref();
			&node.elem
		})
	}
}

#[cfg(test)]
mod tests {
	use super::SList;

	#[test]
	fn append() {
		let l0 = SList::<i32>::new();
		let l1 = l0.append(1);
		let l2 = l0.append(2);

		assert_eq!(l0.head.is_none(), true);
		assert_eq!(l1.head.as_deref().unwrap().elem, 1);
		assert_eq!(l2.head.as_deref().unwrap().elem, 2);
	}

	#[test]
	fn head() {
		let l0 = SList::<i32>::new();
		let l1 = l0.append(1);
		let l2 = l1.append(2);
		let l3 = l1.append(3);

		assert_eq!(l0.head().is_none(), true);
		assert_eq!(*l1.head().unwrap(), 1);
		assert_eq!(*l2.head().unwrap(), 2);
		assert_eq!(*l3.head().unwrap(), 3);
		assert_eq!(*l1.head().unwrap(), 1);
	}

	#[test]
	fn tail() {
		let l0 = SList::<i32>::new();
		let l1 = l0.append(1);
		let l2 = l1.append(2);
		let l3 = l2.append(3);
		let l4 = l1.append(4);
		let adder = |accu : i32, item : &i32| { accu + item };

		assert_eq!(l0.tail().iter().fold(0i32, adder), 0);
		assert_eq!(l1.tail().iter().fold(0i32, adder), 0);
		assert_eq!(l2.tail().iter().fold(0i32, adder), 1);
		assert_eq!(l3.tail().iter().fold(0i32, adder), 3);
		assert_eq!(l4.tail().iter().fold(0i32, adder), 1);
	}

	#[test]
	fn inter() {
		let l0 = SList::<i32>::new();
		let l1 = l0.append(1);
		let l2 = l1.append(2);
		let l3 = l1.append(3);
		let adder = |accu, item| { accu + item };
		
		assert_eq!(l0.iter().fold(0i32, adder), 0);
		assert_eq!(l1.iter().fold(0i32, adder), 1);
		assert_eq!(l2.iter().fold(0i32, adder), 3);
		assert_eq!(l3.iter().fold(0i32, adder), 4);
	}	
}
