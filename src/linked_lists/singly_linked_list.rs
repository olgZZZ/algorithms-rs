use std::fmt::*;

#[ derive( PartialEq, Debug ) ]
pub struct Node< T : Debug > 
{
	pub elem : T,
	pub next : Option< Box< Node< T > > >,
} 
