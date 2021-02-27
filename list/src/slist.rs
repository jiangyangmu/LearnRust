pub struct SList<T> {
	head : SLink<T>,
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
	fn push_peek() {
		let mut list = SList::<i32>::new();

		assert_eq!(list.peek(), None);
		assert_eq!(list.peek_mut(), None);
		list.push(1);
		assert_eq!(list.peek(), Some(&1));
		assert_eq!(list.peek_mut(), Some(&mut 1));
		list.push(2);
		assert_eq!(list.peek(), Some(&2));
		assert_eq!(list.peek_mut(), Some(&mut 2));
		list.peek_mut().map(|elem| { *elem = 4 });
		assert_eq!(list.peek(), Some(&4));
		assert_eq!(list.peek_mut(), Some(&mut 4));
	}
}
