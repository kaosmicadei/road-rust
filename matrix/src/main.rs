use ndarray::{array, ArrayView};
use ndarray::ShapeBuilder;

fn main() {
    let v = [0, 1, 2, 3, 4, 5, 6, 7];

    let arr = ArrayView::from_shape((2, 4), &v).unwrap();
    // This defines an array in C style (row-major) with strides (4,1).
    println!("{}\n", arr);
    
    let arr1 = ArrayView::from_shape((2, 4).strides((1,2)), &v).unwrap();
    // This defines an array in Fortran/Julia style (column-major).
    println!("{:?}\n", arr1);

    let arr2 = ArrayView::from_shape((2, 4).f(), &v).unwrap();
    // This *also* defines an array in Fortran/Julia style (column-major).
    println!("{:?}\n", arr2);
    
    let m = array![[1, 2], [3, 4]];
    println!("{}\n", m);

    let r = m.dot(&arr);
    println!("{}\n", r);
    
    let r1 = m.dot(&arr1);
    println!("{}\n", r1);

    let tensor = ArrayView::from_shape((2, 2, 2), &v).unwrap();
    // This *also* defines an array in C style (row-major).
    println!("tensor:\n{:?}\n", tensor);

    let tensor1 = ArrayView::from_shape((2, 2, 2).f(), &v).unwrap();
    // This *also* defines an array in Fortran/Julia style (column-major).
    println!("tensor:\n{:?}\n", tensor1);
}
