use bit_vec::BitVec; // 0.5.1
 
fn main() {
    let bv = BitVec::from_bytes(&[0b01110100, 0b10010010]);
    assert_eq!(bv.iter().filter(|x| *x).count(), 7);
    assert_eq!(rank(&bv, 4), 3);
    assert_eq!(bv.rank(4), 3);
}
 
fn rank(bv: &BitVec, i: usize) -> usize {
    bv.iter().take(i).filter(|x| *x).count()
}
 
trait Rank {
    fn rank(&self, i: usize) -> usize;
}
 
impl Rank for BitVec {
    fn rank(&self, i: usize) -> usize {
        self.iter().take(i).filter(|x| *x).count()
    }
}
