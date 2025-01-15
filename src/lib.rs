mod pipeline;
mod pipeline_call;

pub use pipeline::*;
pub use pipeline_call::*;

pub trait Pipe: Sized {
    fn pipe<Out, Types, F>(self, f: impl Into<Pipeline<Types, F>>) -> Out
    where
        Pipeline<Types, F>: PipelineCall<Self, Out>;

    fn pipe_ref<'a, Out, TRef, F>(&'a self, f: F) -> Out
    where
        Self: AsRef<TRef>,
        TRef: ?Sized + 'a,
        F: FnOnce(&'a TRef) -> Out;

    fn pipe_mut<'a, Out, TMut, F>(&'a mut self, f: F) -> Out
    where
        Self: AsMut<TMut>,
        TMut: ?Sized + 'a,
        F: FnOnce(&'a mut TMut) -> Out;
}

impl<T> Pipe for T
where
    T: Sized,
{
    fn pipe<Out, Types, F>(self, f: impl Into<Pipeline<Types, F>>) -> Out
    where
        Pipeline<Types, F>: PipelineCall<Self, Out>,
    {
        f.into().call(self)
    }

    fn pipe_ref<'a, Out, TRef, F>(&'a self, f: F) -> Out
    where
        Self: AsRef<TRef>,
        TRef: ?Sized + 'a,
        F: FnOnce(&'a TRef) -> Out,
    {
        f(self.as_ref())
    }

    fn pipe_mut<'a, Out, TMut, F>(&'a mut self, f: F) -> Out
    where
        Self: AsMut<TMut>,
        TMut: ?Sized + 'a,
        F: FnOnce(&'a mut TMut) -> Out,
    {
        f(self.as_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Eq, PartialEq, Debug)]
    struct A;
    #[derive(Eq, PartialEq, Debug)]
    struct B;
    #[derive(Eq, PartialEq, Debug)]
    struct C;
    #[derive(Eq, PartialEq, Debug)]
    struct D;
    #[derive(Eq, PartialEq, Debug)]
    struct E;

    impl AsRef<A> for A {
        fn as_ref(&self) -> &A {
            self
        }
    }

    impl AsMut<A> for A {
        fn as_mut(&mut self) -> &mut A {
            self
        }
    }

    #[test]
    fn pipe() {
        fn a_to_b(_: A) -> B {
            B
        }
        fn a_to_b_ref(_: &A) -> B {
            B
        }
        fn a_to_b_mut(_: &mut A) -> B {
            B
        }
        fn b_to_c(_: B) -> C {
            C
        }
        fn c_to_d(_: C) -> D {
            D
        }
        fn d_to_e(_: D) -> E {
            E
        }

        let x: B = A.pipe(a_to_b);
        assert_eq!(x, B);

        let x: B = A.pipe_ref(a_to_b_ref);
        assert_eq!(x, B);

        let x: B = A.pipe_mut(a_to_b_mut);
        assert_eq!(x, B);

        let x: C = A.pipe((a_to_b, b_to_c));
        assert_eq!(x, C);

        let x: D = A.pipe((a_to_b, b_to_c, c_to_d));
        assert_eq!(x, D);

        let x: E = A.pipe((a_to_b, b_to_c, c_to_d, d_to_e));
        assert_eq!(x, E);
    }
}
