pub trait Unzip3<A, B, C>
where
    Self: Sized + Iterator<Item = (A, B, C)>,
{
    fn unzip_3<FromA, FromB, FromC>(self) -> (FromA, FromB, FromC)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        FromC: Default + Extend<C>,
    {
        let mut ts: FromA = Default::default();
        let mut us: FromB = Default::default();
        let mut vs: FromC = Default::default();

        self.for_each(|(t, u, v)| {
            ts.extend(Some(t));
            us.extend(Some(u));
            vs.extend(Some(v));
        });

        (ts, us, vs)
    }
}

impl<I, A, B, C> Unzip3<A, B, C> for I
where
    I: Sized + Iterator<Item = (A, B, C)>,
{
    fn unzip_3<FromA, FromB, FromC>(self) -> (FromA, FromB, FromC)
    where
        FromA: Default + Extend<A>,
        FromB: Default + Extend<B>,
        FromC: Default + Extend<C>,
    {
        let mut ts: FromA = Default::default();
        let mut us: FromB = Default::default();
        let mut vs: FromC = Default::default();

        self.for_each(|(t, u, v)| {
            ts.extend(Some(t));
            us.extend(Some(u));
            vs.extend(Some(v));
        });

        (ts, us, vs)
    }
}
