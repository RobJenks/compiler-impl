pub struct Flatten {}

impl Flatten {
    pub fn aab<T0,T1,T2>(x: ((T0,T1),T2)) -> (T0,T1,T2)
        where T0: Clone, T1: Clone, T2: Clone {
        ((x.clone().0).0, (x.clone().0).1, x.clone().1)
    }
    pub fn abb<T0,T1,T2>(x: (T0,(T1,T2))) -> (T0,T1,T2)
        where T0: Clone, T1: Clone, T2: Clone {
        (x.clone().0, (x.clone().1).0, (x.clone().1).1)
    }
    pub fn aaab<T0,T1,T2,T3>(x: ((T0,T1,T2),T3)) -> (T0,T1,T2,T3)
        where T0: Clone, T1: Clone, T2: Clone, T3: Clone {
        ((x.clone().0).0, (x.clone().0).1, (x.clone().0).2, x.clone().1)
    }
    pub fn aabb<T0,T1,T2,T3>(x: ((T0,T1),(T2,T3))) -> (T0,T1,T2,T3)
        where T0: Clone, T1: Clone, T2: Clone, T3: Clone {
        ((x.clone().0).0, (x.clone().0).1, (x.clone().1).0, (x.clone().1).1)
    }
    pub fn abbb<T0,T1,T2,T3>(x: (T0,(T1,T2,T3))) -> (T0,T1,T2,T3)
        where T0: Clone, T1: Clone, T2: Clone, T3: Clone {
        (x.clone().0, (x.clone().1).0, (x.clone().1).1, (x.clone().1).2)
    }

    pub fn aabc<T0,T1,T2,T3>(x: (((T0,T1),T2),T3)) -> (T0,T1,T2,T3)
        where T0: Clone, T1: Clone, T2: Clone, T3: Clone {
        (((x.clone().0).0).0, ((x.clone().0).0).1, ((x.clone().0).1), (x.clone().1))
    }

}