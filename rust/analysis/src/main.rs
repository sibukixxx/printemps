extern crate csv;
extern crate nalgebra;

use std::vec::Vec;
use nalgebra::{DVec, DMat, Inv, Mean, ColSlice, Iterable};

fn main() {

    // R の "trees" データを使用
    let data = "Girth,Height,Volume
8.3,70,10.3
8.6,65,10.3
8.8,63,10.2
10.5,72,16.4
10.7,81,18.8
10.8,83,19.7
11.0,66,15.6
11.0,75,18.2
11.1,80,22.6
11.2,75,19.9
11.3,79,24.2
11.4,76,21.0
11.4,76,21.4
11.7,69,21.3
12.0,75,19.1
12.9,74,22.2
12.9,85,33.8
13.3,86,27.4
13.7,71,25.7
13.8,64,24.9
14.0,78,34.5
14.2,80,31.7
14.5,74,36.3
16.0,72,38.3
16.3,77,42.6
17.3,81,55.4
17.5,82,55.7
17.9,80,58.3
18.0,80,51.5
18.0,80,51.0
20.6,87,77.0";

    let mut ncols = 0;

    // http://burntsushi.net/rustdoc/csv/
    let mut reader = csv::Reader::from_string(data).has_headers(true);

    let mut x: Vec<f64> = vec![];
    for row in reader.decode() {
        let vals: Vec<f64> = row.unwrap();
        // 1 列目を目的変数, 残りは説明変数とする
        for i in 0 .. vals.len() {
            x.push(vals[i]);
        }
        // 説明変数の数 (各行は必ず同数のデータを含むと仮定)
        ncols = vals.len();
    }

    // レコード数
    let nrows: usize = x.len() / ncols;

    // ベクトルから nrows x ncols の DMat (動的サイズの行列) を作成
    // http://nalgebra.org/doc/nalgebra/struct.DMat.html
    let dx = DMat::from_row_vec(nrows, ncols, &x);

    // 重回帰
    let coefs = lm_fit(&dx);
    println!("結果 {:?}", &coefs);
}

fn lm_fit(data: &DMat<f64>) -> Vec<f64> {
    // 重回帰

    // 列ごとの平均値を計算
    let means = data.mean();

    let nrows = data.nrows();
    let nfeatures = data.ncols() - 1;

    // 偏差平方和積和行列
    // DMat::from_fn で、行番号 i, 列番号 j を引数とする関数から各要素の値を生成できる
    let smx = DMat::from_fn(nfeatures, nfeatures,
                            |i, j| sum_square(&data.col_slice(i + 1, 0, nrows),
                                              &data.col_slice(j + 1, 0, nrows),
                                              means[i+1], means[j+1]));
    // 偏差積和行列
    // DMat と Vec では演算ができないため、こちらも一列の DMat として生成
    let smy = DMat::from_fn(nfeatures, 1,
                            |i, j| sum_square(&data.col_slice(i + 1, 0, nrows),
                                              &data.col_slice(0, 0, nrows),
                                              means[i], means[0]));
    // 偏回帰係数を計算し、Vec に変換
    let mut res = (smx.inv().unwrap() * smy).to_vec();

    // 切片を計算し、0 番目の要素として挿入
    let intercept = (1..means.len()).fold(means[0], |m, i| m - res[i-1]*means[i]);
    res.insert(0, intercept);
    return res
}

fn sum_square(vec1: &DVec<f64>, vec2: &DVec<f64>, m1: f64, m2: f64) -> f64 {
    // 平方和
    let mut val: f64 = 0.;
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        val += (v1 - m1) * (v2 - m2);
    }
    return val;
}