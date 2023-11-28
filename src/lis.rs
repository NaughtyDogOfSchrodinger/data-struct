use std::cmp::{max, Ordering};

#[allow(unused)]
pub fn lis(array: &[i32]) -> Vec<i32> {
    let mut dp = [1; 32];
    for big_index in (1..array.len()) {
        for small_index in (0..big_index) {
            if array[big_index] > array[small_index] {
                dp[big_index] = max(dp[big_index], dp[small_index] + 1);
            }
        }
    }
    let mut biggest_len = 1;
    for val in dp.into_iter() {
        if val > biggest_len {
            biggest_len = val;
        }
    }
    dp.to_vec()
}

#[allow(unused)]
pub fn lis_1(array: &[i32]) -> i32 {
    let mut dp = [1; 32];
    for big_index in (1..array.len()) {
        for small_index in (0..big_index) {
            if array[big_index] > array[small_index] {
                dp[big_index] = max(dp[big_index], dp[small_index] + 1);
            }
        }
    }
    let mut biggest_len = 1;
    for val in dp.into_iter() {
        if val > biggest_len {
            biggest_len = val;
        }
    }
    biggest_len
}

#[allow(unused)]
pub fn max_envelopes(envelope_size: &mut [[i32; 2]]) -> i32 {
    envelope_size.sort_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Equal => b[1].cmp(&a[1]),
        width_cmp => width_cmp,
    });
    lis_1(
        envelope_size
            .iter()
            .map(|[w, h]| *h)
            .collect::<Vec<i32>>()
            .as_mut_slice(),
    )
}

#[allow(unused)]
pub fn max_envelopes_1(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelope_size = envelopes;
    envelope_size.sort_by(|a, b| match a[0].cmp(&b[0]) {
        Ordering::Equal => b[1].cmp(&a[1]),
        width_cmp => width_cmp,
    });
    let array: Vec<i32> = envelope_size
        .iter()
        .map(|v| *v.get(1).unwrap())
        .collect::<Vec<i32>>();
    //todo 不理解，最长递增子序列
    let mut top = &mut array.clone()[..];
    top.fill(0);
    // 牌堆数初始化为 0
    let mut piles: i32 = 0;
    for poker in &array {

        /***** 搜索左侧边界的二分查找 *****/
        let mut left = 0;
        let mut right = piles;
        while (left < right) {
            let mid = (left + right) / 2;
            match top[mid as usize] - poker {
                sub if sub > 0 => right = mid,
                sub if sub < 0 => left = mid + 1,
                _ => right = mid,
            }
        }
        /*********************************/

        // 没找到合适的牌堆，新建一堆
        if left == piles {
            piles += 1;
        }
        // 把这张牌放到牌堆顶
        top[left as usize] = *poker;
    }
    // 牌堆数就是 LIS 长度
    piles
}

#[cfg(test)]
mod test {
    use crate::lis::{lis, max_envelopes, max_envelopes_1};

    #[test]
    fn test() {
        // let a: &mut [[i32; 2]] = &mut [[5, 4], [6, 4], [6, 7], [2, 3], [5, 2], [1, 8]];
        println!("{:?}", max_envelopes_1(vec![]));
    }
}
