use crate::linked_lists::sixth::{ LinkedList, Link, };
use std::marker::PhantomData;

pub struct CursorMut< 'a, T >
{
	cur : Link< T >,
	list : &'a mut LinkedList< T >,
	index : Option< usize >, 
}

impl< T > LinkedList< T >
{
	pub fn cursor_mut( &mut self ) -> CursorMut< T >
	{
		CursorMut
		{
			list : self,
			cur : None,
			index : None,
		}
	}
}

impl< 'a, T > CursorMut< 'a, T >
{
	pub fn index( &self ) -> Option< usize >
	{
		self.index
	}

	pub fn move_next( &mut self )
	{
		if let Some( cur ) = self.cur
		{
			unsafe
			{
				self.cur = ( *cur.as_ptr() ).back;
				if self.cur.is_some()
				{
					*self.index.as_mut().unwrap() += 1;
				}
				else
				{
					self.index = None;
				}
			}
		}
		else if !self.list.is_empty()
		{
      self.cur = self.list.front;
      self.index = Some( 0 )
		} 
		else {}
	}

	pub fn move_prev( &mut self )
	{
		if let Some( cur ) = self.cur
		{
			unsafe
			{
				self.cur = ( *cur.as_ptr() ).front;
				if self.cur.is_some()
				{
					*self.index.as_mut().unwrap() -= 1; 
				}
				else
				{
					self.index = None;
				}
			}
		}
		else if !self.list.is_empty()
		{
			self.cur = self.list.back;
			self.index = Some( self.list.len - 1 )
		}
		else {}
	}

  pub fn current( &mut self ) -> Option< &mut T >
  {
    unsafe
    {
      self.cur.map( | node | &mut ( *node.as_ptr() ).elem )
    }
  }

  pub fn peek_next( &mut self ) -> Option< &mut T >
  {
    unsafe
    {
      let next = if let Some( cur ) = self.cur
      {
        ( *cur.as_ptr() ).back
      }
      else
      {
        self.list.front
      };

      next.map( | node | &mut ( *node.as_ptr() ).elem )
    }
  }

  pub fn peek_prev( &mut self ) -> Option< &mut T >
  {
    unsafe
    {
      let prev = if let Some( cur ) = self.cur
      {
        ( *cur.as_ptr() ).front
      }
      else
      {
        self.list.back
      };

      prev.map( | node | &mut ( *node.as_ptr() ).elem )
    }
  }

  pub fn split_before( &mut self ) -> LinkedList< T >
  {
  	if let Some( cur ) = self.cur
  	{
  		unsafe
  		{ 
  			// Current state
  			let old_len = self.list.len;
  			let old_idx = self.index.unwrap();
  			let prev = ( *cur.as_ptr() ).front;

  			// What self will become
        let new_len = old_len - old_idx;
        let new_front = self.cur;
        let new_back = self.list.back;
        let new_idx = Some( 0 );

        // What the output will become
        let output_len = old_len - new_len;
        let output_front = self.list.front;
        let output_back = prev;

        // Break the links between cur and prev
        if let Some( prev ) = prev
        {
          ( *cur.as_ptr() ).front = None;
          ( *prev.as_ptr() ).back = None;
  		  }
        
        // Produce the result:
        self.list.len = new_len;
        self.list.front = new_front;
        self.list.back = new_back;
        self.index = new_idx;

        LinkedList
        {
          front: output_front,
          back: output_back,
          len: output_len,
          _pdt: PhantomData,
        }
  		}  
  	}
  	else
  	{
  		std::mem::replace( self.list, LinkedList::new() )
  	}
  }

  pub fn split_after( &mut self ) -> LinkedList< T >
  {
  	if let Some( cur ) = self.cur
  	{
  		unsafe
  		{
  			// Current state
        let old_len = self.list.len;
        let old_idx = self.index.unwrap();
        let next = ( *cur.as_ptr() ).back;
                
        // What self will become
        let new_len = old_idx + 1;
        let new_back = self.cur;
        let new_front = self.list.front;
        let new_idx = Some( old_idx );

        // What the output will become
        let output_len = old_len - new_len;
        let output_front = next;
        let output_back = self.list.back;

        // Break the links between cur and next
        if let Some(next) = next
        {
          ( *cur.as_ptr() ).back = None;
          ( *next.as_ptr() ).front = None;
        }

        // Produce the result:
        self.list.len = new_len;
        self.list.front = new_front;
        self.list.back = new_back;
        self.index = new_idx;

        LinkedList
        {
          front: output_front,
          back: output_back,
          len: output_len,
          _pdt: PhantomData,
        }
  		}
  	}
  	else
  	{
      std::mem::replace( self.list, LinkedList::new() )
  	}
  }

  pub fn splice_before( &mut self, mut input : LinkedList< T > )
  {
  	unsafe
  	{
  		if input.is_empty()
  		{
  			// Input is empty, do nothing.
  		}
  		else if let Some( cur ) = self.cur
  		{
  			// Both lists are non-empty
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        if let Some( prev ) = ( *cur.as_ptr() ).front
        {
          // General Case, no boundaries, just internal fixups
          ( *prev.as_ptr() ).back = Some( in_front );
          ( *in_front.as_ptr() ).front = Some( prev );
          ( *cur.as_ptr() ).front = Some( in_back );
          ( *in_back.as_ptr() ).back = Some( cur );
        }
        else
        {
          // No prev, we're appending to the front
          ( *cur.as_ptr() ).front = Some( in_back );
          ( *in_back.as_ptr() ).back = Some( cur );
          self.list.front = Some( in_front );
        }
        // Index moves forward by input length
        *self.index.as_mut().unwrap() += input.len;
  		}
  		else if let Some( back ) = self.list.back
  		{
  			// We're on the ghost but non-empty, append to the back
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        ( *back.as_ptr() ).back = Some( in_front );
        ( *in_front.as_ptr() ).front = Some( back );
        self.list.back = Some( in_back );
  		}
  		else
  		{
  			std::mem::swap( self.list, &mut input );
  		}

  		self.list.len += input.len;
  		// Not necessary but Polite To Do
  		input.len = 0;

  		// Input dropped here
  	}
  }

  pub fn splice_after( &mut self, mut input : LinkedList< T > )
  {
  	unsafe
  	{
  		// We can either `take` the input's pointers or `mem::forget`
      // it. Using `take` is more responsible in case we ever do custom
      // allocators or something that also needs to be cleaned up!
  		if input.is_empty()
  		{
  			// Input is empty, do nothing.
  		}
  		else if let Some( cur ) = self.cur
  		{
  			// Both lists are non-empty
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        if let Some( next ) = ( *cur.as_ptr() ).back
        {
        	// General Case, no boundaries, just internal fixups
          ( *next.as_ptr() ).front = Some( in_back );
          ( *in_back.as_ptr() ).back = Some( next );
          ( *cur.as_ptr() ).back = Some( in_front );
          ( *in_front.as_ptr() ).front = Some( cur );
        }
        else
        {
        	// No next, we're appending to the back
          ( *cur.as_ptr() ).back = Some( in_front );
          ( *in_front.as_ptr() ).front = Some( cur );
          self.list.back = Some( in_back );
        }
  		} // Index doesn't change
  		else if let Some( front ) = self.list.front
  		{
        // We're on the ghost but non-empty, append to the front
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        ( *front.as_ptr() ).front = Some( in_back );
        ( *in_back.as_ptr() ).back = Some( front );
        self.list.front = Some( in_front );
  		}
  		else
  		{
  			// We're empty, become the input, remain on the ghost
        std::mem::swap( self.list, &mut input );
  		}

  		self.list.len += input.len;
  		// Not necessary but Polite To Do
  		input.len = 0;

  		// Input dropped here 		
  	}
  }
}








