trait areaType{
    fn areaTypesPrint(&self);
}struct longBox {
    width:u64,
    long:u64,
}
struct squareBox {
    width:u64,
    long:u64,
}
impl areaType for longBox{
    fn areaTypesPrint(&self){
        println!("area is(长方形的面积是） {}",self.long*self.width);
    }
}
impl areaType for squareBox{
    fn areaTypesPrint(&self){
        println!("area is(正方形的面积是) {}",self.long*self.width);
    }
}
fn areaPtint<T:areaType>(item:T){
    item.areaTypesPrint();
}
fn main() {
    let longBox1 = longBox {width:100,long:100};
    let squareBox1 = squareBox{width:200,long:200};
    areaPtint(longBox1);
    areaPtint(squareBox1);
}