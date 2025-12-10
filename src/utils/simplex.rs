// based on solution by u/RussellDash332
// https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt988z4/?context=3

const INFINITY: f64 = f64::INFINITY;
const EPS: f64 = 1e-9;

pub fn simplex(lhs: &[Vec<f64>], c: &[f64]) -> (f64, Option<Vec<f64>>) {
    let m = lhs.len();
    let n = lhs[0].len() - 1;

    let mut n_indices: Vec<i32> = (0..n as i32).collect();
    n_indices.push(-1);

    let mut b_indices: Vec<i32> = (n as i32..(n + m) as i32).collect();

    let mut d = vec![vec![0.0; n + 2]; m + 2];

    for (d_row, lhs_row) in d.iter_mut().zip(lhs.iter()) {
        d_row[..=n].copy_from_slice(lhs_row);
        d_row[n + 1] = -1.0;
    }

    for row in d.iter_mut().take(m) {
        row.swap(n, n + 1);
    }

    d[m][..n].copy_from_slice(&c[..n]);
    d[m + 1][n] = 1.0;

    let pivot =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, r: usize, s: usize| {
            let k = 1.0 / d[r][s];

            for i in 0..m + 2 {
                if i == r {
                    continue;
                }
                for j in 0..n + 2 {
                    if j != s {
                        d[i][j] -= d[r][j] * d[i][s] * k;
                    }
                }
            }

            for val in d[r].iter_mut() {
                *val *= k;
            }
            for row in d.iter_mut() {
                row[s] *= -k;
            }
            d[r][s] = k;

            std::mem::swap(&mut b_idx[r], &mut n_idx[s]);
        };

    let find =
        |d: &mut Vec<Vec<f64>>, b_idx: &mut Vec<i32>, n_idx: &mut Vec<i32>, p_idx: usize| -> bool {
            loop {
                let mut best_s = usize::MAX;
                let mut best_val = (INFINITY, i32::MAX);

                for i in 0..=n {
                    if p_idx != 0 || n_idx[i] != -1 {
                        let val = d[m + p_idx][i];
                        let key = (val, n_idx[i]);
                        if best_s == usize::MAX
                            || key.0 < best_val.0 - EPS
                            || ((key.0 - best_val.0).abs() <= EPS && key.1 < best_val.1)
                        {
                            best_s = i;
                            best_val = key;
                        }
                    }
                }
                let s = best_s;

                if d[m + p_idx][s] > -EPS {
                    return true;
                }

                let mut best_r = usize::MAX;
                let mut best_r_key = (INFINITY, i32::MAX);

                for i in 0..m {
                    if d[i][s] > EPS {
                        let ratio = d[i][n + 1] / d[i][s];
                        let key = (ratio, b_idx[i]);
                        if best_r == usize::MAX
                            || key.0 < best_r_key.0 - EPS
                            || ((key.0 - best_r_key.0).abs() <= EPS && key.1 < best_r_key.1)
                        {
                            best_r = i;
                            best_r_key = key;
                        }
                    }
                }
                let r = best_r;

                if r == usize::MAX {
                    return false;
                }

                pivot(d, b_idx, n_idx, r, s);
            }
        };

    let mut split_r = 0;
    let mut min_val = d[0][n + 1];
    for (i, row) in d.iter().enumerate().take(m).skip(1) {
        if row[n + 1] < min_val {
            min_val = row[n + 1];
            split_r = i;
        }
    }

    if d[split_r][n + 1] < -EPS {
        pivot(&mut d, &mut b_indices, &mut n_indices, split_r, n);
        if !find(&mut d, &mut b_indices, &mut n_indices, 1) || d[m + 1][n + 1] < -EPS {
            return (-INFINITY, None);
        }
        for i in 0..m {
            if b_indices[i] == -1 {
                let mut best_s = 0;
                let mut best_key = (d[i][0], n_indices[0]);
                for j in 1..n {
                    let key = (d[i][j], n_indices[j]);
                    if key.0 < best_key.0 - EPS
                        || ((key.0 - best_key.0).abs() <= EPS && key.1 < best_key.1)
                    {
                        best_s = j;
                        best_key = key;
                    }
                }
                pivot(&mut d, &mut b_indices, &mut n_indices, i, best_s);
            }
        }
    }

    if find(&mut d, &mut b_indices, &mut n_indices, 0) {
        let mut x = vec![0.0; n];
        for i in 0..m {
            if b_indices[i] >= 0 && (b_indices[i] as usize) < n {
                x[b_indices[i] as usize] = d[i][n + 1];
            }
        }
        let mut sum_val = 0.0;
        for i in 0..n {
            sum_val += c[i] * x[i];
        }
        return (sum_val, Some(x));
    }

    (-INFINITY, None)
}

pub fn solve_ilp_bnb(initial_a: Vec<Vec<f64>>, obj_coeffs: &[f64]) -> f64 {
    let mut best_val = INFINITY;
    let mut stack = Vec::new();
    stack.push(initial_a);

    while let Some(current_a) = stack.pop() {
        let (val, x_opt) = simplex(&current_a, obj_coeffs);

        if val == -INFINITY || val >= best_val - EPS {
            continue;
        }

        let mut fractional_idx = None;
        let mut fractional_val = 0.0;

        if let Some(x) = x_opt {
            for (i, &xv) in x.iter().enumerate() {
                if (xv - xv.round()).abs() > EPS {
                    fractional_idx = Some(i);
                    fractional_val = xv;
                    break;
                }
            }

            if let Some(idx) = fractional_idx {
                let floor_v = fractional_val.floor();
                let n_cols = current_a[0].len();

                let mut row1 = vec![0.0; n_cols];
                row1[idx] = 1.0;
                row1[n_cols - 1] = floor_v;
                let mut a1 = current_a.clone();
                a1.push(row1);
                stack.push(a1);

                let ceil_v = fractional_val.ceil();
                let mut row2 = vec![0.0; n_cols];
                row2[idx] = -1.0;
                row2[n_cols - 1] = -ceil_v;
                let mut a2 = current_a.clone();
                a2.push(row2);
                stack.push(a2);
            } else if val < best_val {
                best_val = val;
            }
        }
    }

    best_val
}
