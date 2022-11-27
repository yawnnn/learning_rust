use rand::Rng;

const MAX: i32 = 25;

fn rand_fill_vec(vec: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();

    for _ in 0..MAX {
        vec.push(rng.gen_range(0..MAX * 5));
    }
}

fn binary_search(vec: &[i32], n: i32) -> i32 {
    let mut low: usize = 0;
    let mut mid: usize = 0;
    let mut high: usize = vec.len() - 1;

    while low <= high {
        mid = low + (high - low) / 2;

        if vec[mid] > n {
            high = mid - 1;
        } else if vec[mid] < n {
            low = mid + 1;
        } else {
            break;
        }
    }

    return mid as i32;
}

fn run_test(vec: &mut Vec<i32>, n: i32) {
    let index;
   
    vec.sort();
    /* Printing indexes for readibility */
    println!("{:>3?}", (0..vec.len() as i32).collect::<Vec<i32>>());
    println!("{:>3?}", &vec);

    index = binary_search(vec.as_slice(), n);
    println!("val: {}, index: {}", n, index);
}

pub fn main_binary_search() {
    let mut vec: Vec<i32> = Vec::new();
    let n: i32;

    rand_fill_vec(&mut vec);
    n = vec[rand::thread_rng().gen_range(0..MAX) as usize];
    run_test(&mut vec, n);
}