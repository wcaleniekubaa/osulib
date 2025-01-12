pub struct OkMap<F, I, O, Ok, Err>
where
    I: Iterator<Item = Result<Ok, Err>>,
    F: FnMut(Ok) -> O,
{
    iter: I,
    f: F,
}

pub struct ErrMap<F, I, O, Ok, Err>
where
    I: Iterator<Item = Result<Ok, Err>>,
    F: FnMut(Err) -> O,
{
    iter: I,
    f: F,
}

impl<F, I, O, Ok, Err> Iterator for OkMap<F, I, O, Ok, Err>
where
    I: Iterator<Item = Result<Ok, Err>>,
    F: FnMut(Ok) -> O,
{
    type Item = Result<O, Err>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.map(&mut self.f))
    }
}

impl<F, I, O, Ok, Err> Iterator for ErrMap<F, I, O, Ok, Err>
where
    I: Iterator<Item = Result<Ok, Err>>,
    F: FnMut(Err) -> O,
{
    type Item = Result<Ok, O>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.map_err(&mut self.f))
    }
}

pub trait OkMapExt<O, Ok, Err>
where
    Self: Iterator<Item = Result<Ok, Err>>,
{
    #[inline]
    fn ok_map<F>(self, f: F) -> OkMap<F, Self, O, Ok, Err>
    where
        Self: Sized,
        F: FnMut(Ok) -> O,
    {
        OkMap { iter: self, f }
    }
}

impl<I, O, Ok, Err> OkMapExt<O, Ok, Err> for I
where
    I: Iterator<Item = Result<Ok, Err>>,
{
    #[inline]
    fn ok_map<F>(self, f: F) -> OkMap<F, Self, O, Ok, Err>
    where
        Self: Sized,
        F: FnMut(Ok) -> O,
    {
        OkMap { iter: self, f }
    }
}

pub struct AndThen<F, I, O, Ok, Err>
where
    I: Iterator<Item = Result<Ok, Err>>,
    F: FnMut(Ok) -> Result<O, Err>,
{
    iter: I,
    f: F,
}

impl<F, I, O, Ok, Err> Iterator for AndThen<F, I, O, Ok, Err>
where
    I: Iterator<Item = Result<Ok, Err>>,
    F: FnMut(Ok) -> Result<O, Err>,
{
    type Item = Result<O, Err>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| x.and_then(&mut self.f))
    }
}

pub trait AndThenExt<O, Ok, Err>
where
    Self: Iterator<Item = Result<Ok, Err>>,
{
    #[inline]
    fn and_then<F>(self, f: F) -> AndThen<F, Self, O, Ok, Err>
    where
        Self: Sized,
        F: FnMut(Ok) -> Result<O, Err>,
    {
        AndThen { iter: self, f }
    }
}

impl<I, O, Ok, Err> AndThenExt<O, Ok, Err> for I
where
    I: Iterator<Item = Result<Ok, Err>>,
{
    #[inline]
    fn and_then<F>(self, f: F) -> AndThen<F, Self, O, Ok, Err>
    where
        Self: Sized,
        F: FnMut(Ok) -> Result<O, Err>,
    {
        AndThen { iter: self, f }
    }
}
