use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // rng.sample_iter()は乱数を無限に生成するイテレータを返す
    // take(n)はもとのイテレータから最初のn要素だけを取り出すイテレータを返す
    // collect()はイテレータから値を収集して、ベクタやハッシュマップのようなコレクションに格納する。
    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    // windows(2)は元のイテレータから1要素刻みで2要素ずつ値を取り出す
    // 新しいイテレータを返す。例えば元が[1, 2, 3, 4]なら[1, 2], [2, 3], [3, 4]を順に返す
    // all(..)はイテレータから値（例： [1, 2]）を取り出し、クロージャを渡す
    // クロージャがfalseを返したら、そこで処理を打ち切りfalseを返す
    // クロージャがtrueを返している間は、イテレータから次の値を取り出し
    // クロージャへ与え続ける。イテレータの値が尽きるまで（Noneになるまで）
    // クロージャが一度もfalseを返さなかったら、all（..）はtrueを返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
