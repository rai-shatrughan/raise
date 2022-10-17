mod example;
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut my_float_vec = vec![1.1, 2.1, 3.1, 4.1, 5.1, 6.1, 7.1, 8.1, 9.1, 10.1];
    example::loops::loop_example(0, 10);
    example::loops::while_example(0, 10);
    example::loops::for_example(0, 10);
    // example::loops::transform_vec_integer_example(&mut my_vec, 5);
    // example::loops::transform_vec_float_example(&mut my_float_vec, 5.0);

    example::loops::transform_vec_example(&mut my_vec, 5);
    example::loops::transform_vec_example(&mut my_float_vec, 5.0);
    
    example::loops::vec_example(&my_vec);
    example::loops::vec_example(&my_float_vec);
}
