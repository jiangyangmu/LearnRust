pub struct SList<T> {
	head : SLink<T>,
}

pub struct IntoIter<T>{
	next : SList<T>
}
pub struct Iter<'a, T> {
	next : Option<&'a SNode<T>>
}
pub struct IterMut<'a, T> {
	next : Option<&'a mut SNode<T>>
}

type SLink<T> = Option<Box<SNode<T>>>;

struct SNode<T> {
	next : SLink<T>,
	elem : T,
}

impl<T> SList<T> {
	pub fn new() -> Self {
		SList { head : None }
	}

	pub fn push(&mut self, elem: T) {
		let new_node = Box::new(SNode {
			next: self.head.take(),
			elem: elem,
		});
		self.head = Some(new_node);
	}

	pub fn pop(&mut self) -> Option<T> {
		match self.head.take() {
			None => None,
			Some(node) => {
				self.head = node.next;
				Some(node.elem)
			}
		}
	}

	pub fn peek(&self) -> Option<&T> {
		match &self.head {
			None => None,
			Some(node) => Some(&node.elem),
		}
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		match &mut self.head {
			None => None,
			Some(node) => Some(&mut node.elem),
		}
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter { next: self }
	}

	pub fn iter(&self) -> Iter<'_, T> {
		Iter { next: self.head.as_deref() }
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, T> {
		IterMut { next: self.head.as_deref_mut() }
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.pop()
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

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_deref_mut();
			&mut node.elem
		})
	}
}

#[cfg(test)]
mod tests {
	use super::SList;

	#[test]
	fn push_pop() {
		let mut list = SList::<i32>::new();

		list.push(1);
		list.push(2);
		list.push(3);		
		assert_eq!(list.pop(), Some(3));
		list.push(4);
		assert_eq!(list.pop(), Some(4));
		assert_eq!(list.pop(), Some(2));
		list.push(5);
		assert_eq!(list.pop(), Some(5));
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn peek() {
		let mut list = SList::<i32>::new();

		assert_eq!(list.peek(), None);
		list.push(1);
		assert_eq!(list.peek(), Some(&1));
		list.push(2);
		assert_eq!(list.peek(), Some(&2));
	}

	#[test]
	fn peek_mut() {
		let mut list = SList::<i32>::new();

		assert_eq!(list.peek_mut(), None);
		list.push(1);
		assert_eq!(list.peek_mut(), Some(&mut 1));
		list.push(2);
		assert_eq!(list.peek_mut(), Some(&mut 2));
		
		list.peek_mut().map(|elem| { *elem = 4 });
		assert_eq!(list.peek(), Some(&4));
		assert_eq!(list.peek_mut(), Some(&mut 4));
	}

	#[test]
	fn into_inter() {
		let mut list = SList::<i32>::new();

		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.into_iter();

		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn inter() {
		let mut list = SList::<i32>::new();

		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.iter();

		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), None);
		assert_eq!(list.peek(), Some(&3));
	}
	
	#[test]
	fn inter_mut() {
		let mut list = SList::<i32>::new();

		list.push(1);
		list.push(2);
		list.push(3);

		let mut iter = list.iter_mut();

		assert_eq!(iter.next(), Some(&mut 3));
		assert_eq!(iter.next(), Some(&mut 2));
		assert_eq!(iter.next(), Some(&mut 1));
		assert_eq!(iter.next(), None);

		*list.iter_mut().next().unwrap() = 4;
		assert_eq!(*list.peek().unwrap(), 4);
		*list.iter_mut().next().unwrap() = 5;
		assert_eq!(*list.peek().unwrap(), 5);
	}
}
