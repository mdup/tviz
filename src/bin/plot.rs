use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x_rand: Vec<f64> = (0..20).map(|_| rng.gen::<f64>()).collect();
    let y_rand: Vec<f64> = (0..20).map(|_| rng.gen::<f64>()).collect();

    let x_linear: Vec<f64> = (0..40).map(|i| i as f64 / 40.0).collect();
    let y_cube: Vec<f64> = (0..40).map(|j| j as f64 / 40.0).map(|y| y*y*y).collect();

    let x: Vec<f64> = x_rand.into_iter().chain(x_linear.into_iter()).collect();
    let y: Vec<f64> = y_rand.into_iter().chain(y_cube.into_iter()).collect();

    println!("x = {:?}", x);
    println!("y = {:?}", y);

    scatter_plot(x, y);
}

fn scatter_plot(x: Vec<f64>, y: Vec<f64>) {
    let m_x = 16;
    let m_y = 16;
    let n_x = m_x * 2;
    let n_y = m_y * 2;

    let mut a = vec![0; (n_x * n_y)];
    for (&x, &y) in x.iter().zip(y.iter()) {
        let i = (n_x as f64 * x) as usize;
        let j = (n_y as f64 * (1.0 - y)) as usize;
        let idx = i + n_x * j;
        if idx >= a.len() {
            continue;
        }
        a[idx] += 1;
        println!("({}, {})", i, j)
    }

    // for j in 0..n_y {
    //     let row = &a[(j*n_y)..((j+1)*n_y)];
    //     println!("{:?}", row);
    // }

    println!("     y ");
    println!("     ↑ ");
    for jj in 0..m_y {
        if (jj == 0) {
            print!(" 1.0 + ");
        } else {
            print!("     | ");
        }
        for ii in 0..m_x {
            let tl = a[2 * ii     + n_x * 2 * jj] > 0;
            let tr = a[2 * ii + 1 + n_x * 2 * jj] > 0;
            let bl = a[2 * ii     + n_x * (2 * jj + 1)] > 0;
            let br = a[2 * ii + 1 + n_x * (2 * jj + 1)] > 0;
            let c = match (tl, tr, bl, br) {
                (false, false, false, false) => ' ',
                (false, false, false, true)  => '▗',
                (false, false, true, false)  => '▖',
                (false, false, true, true)   => '▄',
                (false, true, false, false)  => '▝',
                (false, true, false, true)   => '▐',
                (false, true, true, false)   => '▞',
                (false, true, true, true)    => '▟',
                (true, false, false, false)  => '▘',
                (true, false, false, true)   => '▚',
                (true, false, true, false)   => '▌',
                (true, false, true, true)    => '▙',
                (true, true, false, false)   => '▀',
                (true, true, false, true)    => '▜',
                (true, true, true, false)    => '▛',
                (true, true, true, true)     => '█',
            };
            print!("{}", c);
        }
        println!();
    }
    println!(" 0.0 +----------------+-→ x ");
    println!("    0.0              1.0 ");
}