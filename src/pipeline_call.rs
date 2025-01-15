use crate::Pipeline;

pub trait PipelineCall<In, Out> {
    fn call(self, x: In) -> Out;
}

impl<T1, T2, F1> PipelineCall<T1, T2> for Pipeline<(T1, T2), F1>
where
    F1: FnOnce(T1) -> T2,
{
    fn call(self, x: T1) -> T2 {
        (self.0)(x)
    }
}

impl<T1, T2, T3, F1, F2> PipelineCall<T1, T3> for Pipeline<(T1, T2, T3), (F1, F2)>
where
    F1: FnOnce(T1) -> T2,
    F2: FnOnce(T2) -> T3,
{
    fn call(self, x: T1) -> T3 {
        (self.0 .1)((self.0 .0)(x))
    }
}

impl<T1, T2, T3, T4, F1, F2, F3> PipelineCall<T1, T4> for Pipeline<(T1, T2, T3, T4), (F1, F2, F3)>
where
    F1: FnOnce(T1) -> T2,
    F2: FnOnce(T2) -> T3,
    F3: FnOnce(T3) -> T4,
{
    fn call(self, x: T1) -> T4 {
        (self.0 .2)((self.0 .1)((self.0 .0)(x)))
    }
}

impl<T1, T2, T3, T4, T5, F1, F2, F3, F4> PipelineCall<T1, T5>
    for Pipeline<(T1, T2, T3, T4, T5), (F1, F2, F3, F4)>
where
    F1: FnOnce(T1) -> T2,
    F2: FnOnce(T2) -> T3,
    F3: FnOnce(T3) -> T4,
    F4: FnOnce(T4) -> T5,
{
    fn call(self, x: T1) -> T5 {
        (self.0 .3)((self.0 .2)((self.0 .1)((self.0 .0)(x))))
    }
}
