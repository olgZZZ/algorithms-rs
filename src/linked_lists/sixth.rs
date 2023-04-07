///
/// A Production Unsafe Deque
///
use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct LinkedList< T >
{
	front : Link< T >,
	back : Link< T >,
	len : usize,
	_pdata : PhantomData< T >,
}

type Link< T > = Option< NonNull< Node< T > > >;

struct Node< T >
{
	front : Link< T >,
	back : Link< T >,
	elem : T,
}

impl< T > LinkedList< T >
{
	pub fn new() -> Self
	{
		Self
		{
			front : None,
			back : None,
			len : 0,
			_pdata : PhantomData,
		}
	}

	pub fn push_front( &mut self, elem : T )
	{
		unsafe
		{
			let new = NonNull::new_unchecked( Box::into_raw( Box::new( Node
			{
				front : None,
				back : None,
				elem,
			})));

			if let Some( old ) = self.front
			{
				( *old.as_ptr() ).front = Some( new );
				( *new.as_ptr() ).back = Some ( old );
			}
			else
			{
			  self.back = Some( new );
			}

			self.front = Some( new );
			self.len += 1;
		}
	}

	pub fn pop_front( &mut self ) -> Option< T >
	{
		unsafe
		{
			self.front.map( | node |
			{
				let boxed_node = Box::from_raw( node.as_ptr() );
				let result = boxed_node.elem;

				self.front = boxed_node.back;
				if let Some( new ) = self.front
				{
					( *new.as_ptr() ).front = None;
				}
				else
				{
					self.back = None;
				}

				self.len -= 1;
				result
			})
		} 
	}

	pub fn len( &self ) -> usize
	{
		self.len
	} 
}

impl< T > Drop for LinkedList< T >
{
	fn drop( &mut self )
	{
		while let Some( _ ) = self.pop_front() {}
	}
}






#[ cfg( test ) ]
mod test
{
	use super::LinkedList;

	#[ test ]
	fn test_basic_front()
	{
		let mut list = LinkedList::new();

		assert_eq!( list.len(), 0 );
		assert_eq!( list.pop_front(), None );
		assert_eq!( list.len(), 0 );

		list.push_front( 10 );
    assert_eq!( list.len(), 1 );
    assert_eq!( list.pop_front(), Some( 10 ) );
    assert_eq!( list.len(), 0 );
    assert_eq!( list.pop_front(), None );
    assert_eq!( list.len(), 0 );

    list.push_front( 10 );
    assert_eq!( list.len(), 1 );
    list.push_front( 20 );
    assert_eq!( list.len(), 2 );
    list.push_front( 30 );
    assert_eq!( list.len(), 3 );
    assert_eq!( list.pop_front(), Some( 30 ) );
    assert_eq!( list.len(), 2 );
    list.push_front( 40 );
    assert_eq!( list.len(), 3 );
    assert_eq!( list.pop_front(), Some( 40 ) );
    assert_eq!( list.len(), 2 );
    assert_eq!( list.pop_front(), Some( 20 ) );
    assert_eq!( list.len(), 1 );
    assert_eq!( list.pop_front(), Some( 10 ) );
    assert_eq!( list.len(), 0 );
    assert_eq!( list.pop_front(), None );
    assert_eq!( list.len(), 0 );
    assert_eq!( list.pop_front(), None );
    assert_eq!( list.len(), 0 );
	}
} 










