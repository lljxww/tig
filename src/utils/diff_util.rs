#[derive(Debug, PartialEq, Eq)]
pub enum DiffLine<'a> {
    /// old 和 new 中都存在的行。
    Context(&'a str),

    /// 只存在于 new 中的行。
    Added(&'a str),

    /// 只存在于 old 中的行。
    Removed(&'a str),
}

/// 比较两个按行切分后的文本，生成 diff。
///
/// 算法：
/// 1. 先剔除公共前缀和公共后缀。
/// 2. 对真正发生变化的中间区域执行 Hirschberg LCS。
///
/// 相比传统完整二维 LCS DP：
///
/// - 时间复杂度：O(N * M)
/// - 额外空间复杂度：O(min(N, M))
///
/// 其中 N、M 是裁剪公共前后缀之后的长度。
pub fn diff_lines<'a>(old: &'a [&'a str], new: &'a [&'a str]) -> Vec<DiffLine<'a>> {
    // 最坏情况下没有任何公共行：
    //
    // old 全部 Removed
    // new 全部 Added
    //
    // 因此结果最大长度不会超过 old.len() + new.len()。
    // 提前申请容量，减少 Vec 扩容。
    let mut result = Vec::with_capacity(old.len() + new.len());

    /*
     * 1. 查找公共前缀
     *
     * old:
     * A
     * B
     * C
     * D
     *
     * new:
     * A
     * B
     * X
     * D
     *
     * A、B 一定是 Context，没有必要参与 LCS 计算。
     */
    let prefix_len = old
        .iter()
        .zip(new.iter())
        .take_while(|(a, b)| a == b)
        .count();

    /*
     * 2. 查找公共后缀
     *
     * 必须限制最大长度，防止 suffix 与 prefix 重叠。
     */
    let max_suffix_len = old.len().min(new.len()) - prefix_len;

    let suffix_len = old[prefix_len..]
        .iter()
        .rev()
        .zip(new[prefix_len..].iter().rev())
        .take(max_suffix_len)
        .take_while(|(a, b)| a == b)
        .count();

    /*
     * 公共前缀直接加入结果。
     */
    result.extend(old[..prefix_len].iter().copied().map(DiffLine::Context));

    /*
     * 3. 只对中间真正发生变化的区域执行 LCS。
     */
    let old_end = old.len() - suffix_len;
    let new_end = new.len() - suffix_len;

    let old_middle = &old[prefix_len..old_end];
    let new_middle = &new[prefix_len..new_end];

    /*
     * Hirschberg 的空间消耗取决于第二个参数 b 的长度，
     * 因为 DP 数组大小为 b.len() + 1。
     *
     * 因此始终让较短的序列作为 b，
     * 将额外空间控制在：
     *
     * O(min(old.len(), new.len()))
     *
     * a_is_old 用来告诉算法：
     *
     * a 中独有的行到底是 Removed 还是 Added。
     */
    if old_middle.len() >= new_middle.len() {
        hirschberg(old_middle, new_middle, true, &mut result);
    } else {
        hirschberg(new_middle, old_middle, false, &mut result);
    }

    /*
     * 4. 公共后缀同样直接加入结果。
     */
    if suffix_len > 0 {
        result.extend(
            old[old.len() - suffix_len..]
                .iter()
                .copied()
                .map(DiffLine::Context),
        );
    }

    result
}

/// Hirschberg LCS 核心。
///
/// a、b 并不固定代表 old/new。
///
/// 为了让 DP 使用更少内存，我们保证：
///
///     b.len() <= a.len()
///
/// `a_is_old` 用来说明 a 的实际语义：
///
/// - true  => a = old, b = new
/// - false => a = new, b = old
///
/// Hirschberg 的核心思想：
///
/// 不保存完整的二维 DP 表，而只保存一行。
///
/// 将 a 从中间切开：
///
///     a = a_left + a_right
///
/// 然后计算：
///
///     LCS(a_left, b[..k])
///     LCS(a_right, b[k..])
///
/// 找出能够让两个 LCS 长度之和最大的 k，
/// 即可确定 LCS 一定可以从这个位置穿过。
///
/// 然后递归处理左右两部分。
fn hirschberg<'a>(
    a: &'a [&'a str],
    b: &'a [&'a str],
    a_is_old: bool,
    result: &mut Vec<DiffLine<'a>>,
) {
    /*
     * 情况 1：
     *
     * a 已经为空。
     *
     * 那么 b 中剩余的所有行都只存在于 b。
     */
    if a.is_empty() {
        push_unique_b(b, a_is_old, result);
        return;
    }

    /*
     * 情况 2：
     *
     * b 已经为空。
     *
     * 那么 a 中剩余的所有行都只存在于 a。
     */
    if b.is_empty() {
        push_unique_a(a, a_is_old, result);
        return;
    }

    /*
     * 情况 3：
     *
     * a 只剩一行。
     *
     * 此时无需继续 DP。
     * 只需要检查这一行是否出现在 b 中。
     */
    if a.len() == 1 {
        let line = a[0];

        if let Some(pos) = b.iter().position(|candidate| *candidate == line) {
            /*
             * b 中匹配位置之前的内容，只存在于 b。
             */
            push_unique_b(&b[..pos], a_is_old, result);

            /*
             * 当前行同时存在于 a 和 b，因此是 Context。
             */
            result.push(DiffLine::Context(line));

            /*
             * b 中匹配位置之后的内容，只存在于 b。
             */
            push_unique_b(&b[pos + 1..], a_is_old, result);
        } else {
            /*
             * 完全没有匹配。
             *
             * 为了让 replacement 的输出更符合常见 diff：
             *
             * -Removed
             * +Added
             *
             * 因此优先输出 Removed。
             */
            if a_is_old {
                push_unique_a(a, a_is_old, result);
                push_unique_b(b, a_is_old, result);
            } else {
                push_unique_b(b, a_is_old, result);
                push_unique_a(a, a_is_old, result);
            }
        }

        return;
    }

    /*
     * 将 a 从中间分成两部分。
     *
     * 每一次递归都会让 a 大约减半，
     * 所以递归深度约为 O(log a.len())。
     */
    let mid = a.len() / 2;

    let (a_left, a_right) = a.split_at(mid);

    /*
     * 找出 b 的最佳切分位置。
     *
     * 这里单独使用一个作用域，
     * 目的是找到 split 后立刻释放两个 DP 数组，
     * 避免它们跨越递归调用继续占用内存。
     */
    let split = {
        /*
         * left[j] 表示：
         *
         * LCS(a_left, b[..j])
         */
        let left = lcs_prefix_lengths(a_left, b);

        /*
         * right[j] 表示反向计算结果：
         *
         * LCS(reverse(a_right), reverse(b)[..j])
         *
         * 因此：
         *
         * right[b.len() - j]
         *
         * 就等价于：
         *
         * LCS(a_right, b[j..])
         */
        let right = lcs_suffix_lengths(a_right, b);

        let mut best_split = 0;
        let mut best_score = 0;

        /*
         * 尝试 b 的所有切分位置：
         *
         * b[..j]
         * b[j..]
         *
         * 选择：
         *
         * LCS(a_left, b[..j])
         * +
         * LCS(a_right, b[j..])
         *
         * 最大的位置。
         */
        for j in 0..=b.len() {
            let score = left[j] + right[b.len() - j];

            if score > best_score {
                best_score = score;
                best_split = j;
            }
        }

        best_split
    };

    let (b_left, b_right) = b.split_at(split);

    /*
     * 顺序非常重要：
     *
     * 先处理左半部分，再处理右半部分，
     * 所以最终生成的 diff 天然保持原始文本顺序，
     * 不需要 result.reverse()。
     */
    hirschberg(a_left, b_left, a_is_old, result);

    hirschberg(a_right, b_right, a_is_old, result);
}

/// 计算：
///
///     LCS(a, b[..j])
///
/// 对每一个 j 的长度。
///
/// 传统二维 DP：
///
///     dp[i][j]
///
/// 这里我们只保留当前这一行：
///
///     dp[j]
///
/// 因此空间从：
///
///     O(a.len() * b.len())
///
/// 降为：
///
///     O(b.len())
fn lcs_prefix_lengths(a: &[&str], b: &[&str]) -> Vec<usize> {
    let mut dp = vec![0usize; b.len() + 1];

    for &a_line in a {
        /*
         * diagonal 保存旧二维 DP 中：
         *
         * dp[i - 1][j - 1]
         */
        let mut diagonal = 0;

        for j in 1..=b.len() {
            /*
             * dp[j] 在被覆盖之前表示：
             *
             * dp[i - 1][j]
             *
             * 即二维 DP 中“上方”的值。
             */
            let up = dp[j];

            if a_line == b[j - 1] {
                /*
                 * 当前两行相同：
                 *
                 * dp[i][j] =
                 * dp[i - 1][j - 1] + 1
                 */
                dp[j] = diagonal + 1;
            } else {
                /*
                 * 当前两行不同：
                 *
                 * dp[i][j] =
                 * max(
                 *     dp[i - 1][j],
                 *     dp[i][j - 1]
                 * )
                 *
                 * up      = dp[i - 1][j]
                 * dp[j-1] = dp[i][j - 1]
                 */
                dp[j] = up.max(dp[j - 1]);
            }

            /*
             * 保存旧的 dp[i - 1][j]，
             * 供下一列作为 diagonal 使用。
             */
            diagonal = up;
        }
    }

    dp
}

/// 与 `lcs_prefix_lengths` 相同，
/// 但从两个序列的末尾向前计算。
///
/// 返回：
///
///     dp[j]
///
/// 表示：
///
///     LCS(reverse(a), reverse(b)[..j])
///
/// Hirschberg 使用它来计算右半部分的 LCS 长度。
fn lcs_suffix_lengths(a: &[&str], b: &[&str]) -> Vec<usize> {
    let mut dp = vec![0usize; b.len() + 1];

    for &a_line in a.iter().rev() {
        let mut diagonal = 0;

        for j in 1..=b.len() {
            let up = dp[j];

            /*
             * b 从末尾开始读取。
             *
             * j = 1 => b[b.len() - 1]
             * j = 2 => b[b.len() - 2]
             */
            let b_line = b[b.len() - j];

            if a_line == b_line {
                dp[j] = diagonal + 1;
            } else {
                dp[j] = up.max(dp[j - 1]);
            }

            diagonal = up;
        }
    }

    dp
}

/// a 中独有的元素。
///
/// a_is_old = true：
///     a 是 old，因此是 Removed。
///
/// a_is_old = false：
///     a 是 new，因此是 Added。
fn push_unique_a<'a>(lines: &'a [&'a str], a_is_old: bool, result: &mut Vec<DiffLine<'a>>) {
    if a_is_old {
        result.extend(lines.iter().copied().map(DiffLine::Removed));
    } else {
        result.extend(lines.iter().copied().map(DiffLine::Added));
    }
}

/// b 中独有的元素。
///
/// b 与 a 的语义刚好相反。
fn push_unique_b<'a>(lines: &'a [&'a str], a_is_old: bool, result: &mut Vec<DiffLine<'a>>) {
    if a_is_old {
        // a = old
        // b = new
        result.extend(lines.iter().copied().map(DiffLine::Added));
    } else {
        // a = new
        // b = old
        result.extend(lines.iter().copied().map(DiffLine::Removed));
    }
}
