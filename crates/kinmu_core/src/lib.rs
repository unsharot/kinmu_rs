//! 入力、生成、出力までの処理を一般化したもの

/// 入力の共通のふるまい
pub trait Input<C> {
    fn load_config(&mut self) -> anyhow::Result<C>;
}

/// 生成の共通のふるまい
pub trait Generator<C, A> {
    fn run(&mut self, config: &C) -> anyhow::Result<A>;
}

/// 出力の共通のふるまい
pub trait Output<A> {
    fn run(&mut self, answer: &A) -> anyhow::Result<()>;
}

/// 入力、生成、出力を実行
pub fn run<C, A, I: Input<C>, G: Generator<C, A>, O: Output<A>>(
    input: &mut I,
    generator: &mut G,
    output: &mut O,
) -> anyhow::Result<()> {
    let config = input.load_config()?;

    let answer = generator.run(&config)?;

    output.run(&answer)?;

    Ok(())
}
