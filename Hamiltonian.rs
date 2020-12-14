use std::ptr;
use std::println;
pub struct Hamiltonian
{
    Width:i16,
    Height:i16,
    Points:Vec<HamiltonianNode>
}
impl Hamiltonian {
    pub unsafe fn new(size:i16) -> Hamiltonian {
        let  Size:i32=size.into();
        let mut CurPos:i32=0;
        let mut CurRow:i32=0;
        let mut CurColumn:i32=0;
        let mut PointsToPass =vec![];
        for y in 0i32..Size
        {
            for x in 0i32..Size
            {
                PointsToPass.push(HamiltonianNode{NextNodeP:ptr::null_mut(),PrevNodeP:ptr::null_mut(),XPos:x as i16,YPos:y as i16,HamiltonianID:-1});
            }
        }
        let mut CurrentNode= &mut PointsToPass[0];
        (*CurrentNode).HamiltonianID=CurPos;
        CurPos+=1;
        let mut LastNode: *mut HamiltonianNode=&mut *CurrentNode;
        let mut GoingLeft=true;
        while CurRow < Size
        {
            if GoingLeft
            {
                CurColumn=1;
                while CurColumn < Size
                {
                    CurrentNode=&mut PointsToPass[(CurColumn+CurRow*Size) as usize];
                    (*CurrentNode).HamiltonianID=CurPos;
                    CurPos+=1;
                    CurColumn+=1;
                    (*CurrentNode).PrevNodeP=LastNode;
                    (*LastNode).NextNodeP=CurrentNode;
                    LastNode = CurrentNode;
                }
            }
            else
            {
                CurColumn=Size-1;
                while CurColumn>0
                {
                    CurrentNode=&mut PointsToPass[(CurColumn+CurRow*Size) as usize];
                    (*CurrentNode).HamiltonianID=CurPos;
                    CurPos+=1;
                    CurColumn-=1;
                    (*CurrentNode).PrevNodeP=LastNode;
                    (*LastNode).NextNodeP=CurrentNode;
                    LastNode = CurrentNode;
                }
            }
            GoingLeft = !GoingLeft;//turn around each loop
            CurRow+=1;
        }
        for y in ((Size-1)..0).rev()
        {
            CurrentNode=&mut PointsToPass[(y * Size) as usize];
            (*CurrentNode).HamiltonianID=CurPos;
            CurPos+=1;
            (*CurrentNode).PrevNodeP=LastNode;
            (*LastNode).NextNodeP=CurrentNode;
            LastNode = CurrentNode;
        }
        PointsToPass[0].PrevNodeP=CurrentNode;
       return Hamiltonian {Width: size, Height: size, Points: PointsToPass}
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