use crate::lesson04::course04::{Time, vec_sum, Rectangle, calc_area};
use crate::lesson04::course04::TrafficLight::{Yellow, Green, Red};

mod lesson04;
mod lesson03;

fn main() {
    //第四课
    lesson04();

    //第三课，启动一个简单的tcp server
    // start_tcp_server();
}

fn lesson04() {
    //显示交通信号灯时间
    show_traffic_light_duration();
    //对集合元素求和
    run_vec_sum();
    //计算面积
    run_calc_area();
}

fn run_calc_area() {
    let rec = Rectangle::new(3.0, 5.0);
    println!("矩形{:#?}的面积为：{}", &rec, calc_area(&rec));
}

fn run_vec_sum() {
    let elements: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}求和结果：{:?}", elements, vec_sum(&elements));


    let elements: Vec<u32> = vec![1, std::u32::MAX];
    println!("{:?}求和结果：{:?}", elements, vec_sum(&elements));
}

fn show_traffic_light_duration() {
    println!("红灯持续时间为：{}", Red.duration());
    println!("绿灯持续时间为：{}", Green.duration());
    println!("黄灯持续时间为：{}", Yellow.duration());
}

fn start_tcp_server() {
    lesson03::simple_tcp_server::start_server();
}
