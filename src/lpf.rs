pub struct LpfStruct<I>
where
    I: Iterator,
{
    iter: I,
    prev: I::Item,
    alpha: f32,
}

impl<I> Iterator for LpfStruct<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let vin = self.iter.next();
        vin.map(|vin| {
            let vout = self.prev;
            let vout = (vout as f32 + self.alpha * (vin - vout) as f32) as i32;
            self.prev = vout;
            vout
        })
    }
}

trait Lpf<I: Iterator> {
    fn lpf(self, gain: f32) -> LpfStruct<I>;
}

impl<I: Iterator<Item = i32>> Lpf<I> for I {
    fn lpf(self, cutoff: f32) -> LpfStruct<I> {
        let rc = 1.0 / (cutoff * 2.0 * core::f32::consts::PI);
        let alpha = 1.0 / (1.0 + rc);
        LpfStruct {
            iter: self,
            prev: 0,
            alpha,
        }
    }
}
