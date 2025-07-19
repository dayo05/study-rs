mod collatz;
mod elevator;
mod fib;
mod geo;
mod matrix;
use crate::collatz::collatz;
use crate::elevator::*;
use crate::fib::fib;
use crate::geo::normalize;
use crate::matrix::transpose;

fn main() {
    println!("Hello, world!");
    let n = 20;
    println!("fib({}) = {}", n, fib(n));
    println!("collatz({}) = {}", n, collatz(n));
    let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("Trnaspose({:#?}) = {:#?}", mat, transpose(mat));

    let point = [1f64, 2f64, 3f64];
    println!("normalize({:?}) = {:?}", point, normalize(point));

    println!(
        "1층 승객이 위쪽 버튼을 눌렀습니다. {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("엘리베이터가 1층에 도착했습니다: {:?}", car_arrived(0));
    println!("엘리베이터 문이 열렸습니다. {:?}", car_door_opened());
    println!(
        "승객이 3층 버튼을 눌렀습니다. {:?}",
        car_floor_button_pressed(3)
    );
    println!("엘리베이터 문이 닫혔습니다: {:?}", car_door_closed());
    println!("엘리베이터가 3층에 도착했습니다. {:?}", car_arrived(3));
}
