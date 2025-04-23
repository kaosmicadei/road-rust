#[allow(unused_imports)]
use ndarray::{array, Array1, ArrayView, ArrayViewMut, Order, ShapeBuilder};

fn bit_swap0(idx: usize, value: usize) -> usize {
    let x = (value ^ (value >> idx)) & 1;
    value ^ ((x << idx) | x)
}

fn main() {
    // let v = [0, 1, 2, 3, 4, 5, 6, 7];

    // let arr = ArrayView::from_shape((2, 4), &v).unwrap();
    // // This defines an array in C style (row-major) with strides (4,1).
    // println!("{}\n", arr);
    
    // let arr1 = ArrayView::from_shape((2, 4).strides((1,2)), &v).unwrap();
    // // This defines an array in Fortran/Julia style (column-major).
    // println!("{:?}\n", arr1);

    // let arr2 = ArrayView::from_shape((2, 4).f(), &v).unwrap();
    // // This *also* defines an array in Fortran/Julia style (column-major).
    // println!("{:?}\n", arr2);

    // let arr3 = ndarray::arr1(&v);
    // // This *also* defines an array in Fortran/Julia style (column-major).
    // println!("{:?}\n", arr3);
    
    // let m = array![[1, 2], [3, 4]];
    // println!("{}\n", m);

    // let r = m.dot(&arr);
    // println!("{}\n", r);
    
    // let r1 = m.dot(&arr1);
    // println!("{}\n", r1);

    // let tensor = ArrayView::from_shape((2, 2, 2), &v).unwrap();
    // // This *also* defines an array in C style (row-major).
    // println!("tensor:\n{:?}\n", tensor);

    // let tensor1 = ArrayView::from_shape((2, 2, 2).f(), &v).unwrap();
    // // This *also* defines an array in Fortran/Julia style (column-major).
    // println!("tensor:\n{:?}\n", tensor1);

    // let s = tensor.as_slice().unwrap();
    // println!("tensor as slice:\n{:?}", s);

    // println!("\n\n=== Time for the cool stuff!!! ===\n");

    let m: Array1<i32> = array![[1, 3], [2, 4]];

    let data = array![1, 2, 3, 4, 5, 6, 7, 8];
    let mut res = data.to_shape((4, 2)).unwrap();
    let mut tmp = res.clone();

    for i in 0..3 {
        // Fast "flatten": access the raw position through slice.
        let raw_view = res.as_slice()  // WARNING! Only works with row major!
            .unwrap();
        let raw_tmp = tmp.as_slice_mut().unwrap();

        raw_tmp
            .iter_mut()
            .enumerate()
            .for_each(|(j, v)| *v = raw_view[bit_swap0(i, j)]);

        let reordered = ArrayView::from_shape((4, 2), &raw_tmp).unwrap();
        res.assign(&reordered.dot(&m));
        println!("\n{}\n", reordered);
    }

    println!("{}", res.flatten_with_order(Order::ColumnMajor));
}
