use crate::IData::IData::IData;
use crate::sharding_engine::Ishard::IShard;


//We process particles using the node tree, here we use 2 nodes because we use O(log n) notation.
// If  use 4 nodes, we will have time confusion and we will have to process here 2 times at compile time.
//Like:
//For i in range {
//   For j in i{

//We will use hashmap
pub struct NodeTree {
    data:IShard<IData>,
    right:None,
    left:None
}