use std::fmt::Debug;

fn main() {
    println!("Quick sort!");
    let mut arr = [ 32, 1, 0, 3, 7, 17, 8, -6, 3, 10 ];
    let size = arr.len();
    quick_sort(&mut arr, 0, size - 1);
}

fn quick_sort<T>(arr: &mut [T], mut lower: usize, mut upper: usize)
    where
        T:PartialOrd+Ord+Sized+Debug+Copy
{
    println!("array before sort {:?}", &arr);

    if upper <= lower {
        return;
    }
    let pivot = arr[lower];
    let start= lower;
    let stop = upper;
    while lower < upper {
        while (arr[lower] <= pivot) && (lower < upper) {
            lower = lower + 1;
        }
        while arr[upper] > pivot && lower <= upper {
            upper = upper - 1;
        }
        if lower < upper {
            arr.swap(lower, upper);
        }
    }
    arr.swap(upper, start);
    if upper > 0 {
        quick_sort(arr, start, upper - 1);
    }
    quick_sort(arr, upper + 1, stop);
}
