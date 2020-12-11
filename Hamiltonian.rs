use std::ptr;
struct Hamiltonian
{
    Width:i16,
    Height:i16,
    Points:Vec<HamiltonianNode>
}
impl Hamiltonian {
    fn new(Size:i16) -> Hamiltonian {
        let mut PointsToPass =vec![];
        let mut CurrentNode= HamiltonianNode{NextNodeP:ptr::null_mut(),PrevNodeP:ptr::null_mut(),XPos:-1,YPos:-1,HamiltonianID:-1};
        let mut LastNode: *mut HamiltonianNode=&mut CurrentNode;
        PointsToPass.push(CurrentNode);
        for y in 0..Size
        {
            for x in 0..Size
            {
                PointsToPass.push(HamiltonianNode{NextNodeP:ptr::null_mut(),PrevNodeP:LastNode,XPos:-1,YPos:-1,HamiltonianID:-1});
                let Node:*mut HamiltonianNode=&mut PointsToPass[(x+y*Size) as usize];
                if x+y*Size-1>0
                {
                PointsToPass[(x+y*Size-1) as usize].NextNodeP=Node;
                }
                LastNode=Node;
            }
        }
        Hamiltonian {Width: Size, Height: Size, Points: PointsToPass}
    }
}
struct HamiltonianNode
{
    NextNodeP:*mut HamiltonianNode,
    PrevNodeP:*mut HamiltonianNode,
    XPos:i16,
    YPos:i16,
    HamiltonianID:i32,
}