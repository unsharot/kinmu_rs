pub trait Input<C> {
    fn load_config(&mut self) -> anyhow::Result<C>;
}

pub trait Generator<C, A> {
    fn run(&mut self, config: &C) -> anyhow::Result<A>;
}

pub trait Output<A> {
    fn run(&mut self, answer: &A) -> anyhow::Result<()>;
}

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
