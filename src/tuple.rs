pub mod tuple {
    use std::ops;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub struct Tuple {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        w: f64,
    }

    impl ops::Sub for Tuple {
        type Output = Tuple;

        fn sub(self, other: Tuple) -> Tuple {
            Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
        }
    }

    impl ops::Add<Self> for Tuple {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
        }
    }

    impl ops::Mul<f64> for Tuple {
        type Output = Self;

        fn mul(self, other: f64) -> Self {
            Tuple::new(self.x * other, self.y * other, self.z * other, self.w)
        }
    }

    impl ops::Neg for Tuple {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Tuple::new(-self.x, -self.y, -self.z, -self.w)
        }
    }

    impl Tuple {
        pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
            Tuple { x, y, z, w }
        }

        fn equals(&self, other: &Tuple) -> bool {
            let e = f64::EPSILON;
            f64::abs(self.x - other.x) < e && f64::abs(self.y - other.y) < e && f64::abs(self.z - other.z) < e && self.w == other.w
        }

        fn add(&self, other: &Tuple) -> Tuple {
            Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
        }

        fn sub(&self, other: &Tuple) -> Tuple {
            Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
        }

        pub fn point(x: f64, y: f64, z: f64) -> Tuple {
            Tuple::new(x, y, z, 1.0)
        }

        pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
            Tuple::new(x, y, z, 0.0)
        }

        fn negate(&self) -> Tuple {
            Tuple::new(-self.x, -self.y, -self.z, -self.w)
        }

        fn multiply(&self, scalar: f64) -> Tuple {
            Tuple::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w)
        }

        fn divide(&self, scalar: f64) -> Tuple {
            Tuple::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w)
        }

        fn magnitude(&self) -> f64 {
            f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
        }

        fn normalize(&self) -> Tuple {
            let m = self.magnitude();
            Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m)
        }

        fn dot(&self, other: &Tuple) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
        }

        fn cross(&self, other: &Tuple) -> Tuple {
            Tuple::vector(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
        }
    }

    #[test]
    fn can_create_tuple() {
        let t = Tuple::new(1.0,2.0, 3.0, 4.0);
        assert_eq!(t.x, 1.0);
        assert_eq!(t.y, 2.0);
        assert_eq!(t.z, 3.0);
        assert_eq!(t.w, 4.0);
    }

    #[test]
    fn vector_is_not_point() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.w, 0.0);
    }

    #[test]
    fn point_is_not_vector() {
        let p = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn can_compare_tuples() {
        let t1 = Tuple::new(1.0,2.0, 3.0, 4.0);
        let t2 = Tuple::new(1.0,2.0, 3.0, 4.0);
        assert!(t1.equals(&t2));
    }

    #[test]
    fn can_add_vectors() {
        let t1 = Tuple::vector(1.0,2.0, 3.0);
        let t2 = Tuple::vector(1.0,2.0, 3.0);
        let t3 = t1.add(&t2);
        assert_eq!(t3.x, 2.0);
        assert_eq!(t3.y, 4.0);
        assert_eq!(t3.z, 6.0);
        assert_eq!(t3.w, 0.0);
        assert_eq!(t1 + t2, Tuple::vector(2.0, 4.0, 6.0));
    }

    #[test]
    fn can_add_vector_to_point() {
        let t1 = Tuple::point(1.0,2.0, 3.0);
        let t2 = Tuple::vector(1.0,2.0, 3.0);
        let t3 = t1.add(&t2);
        assert_eq!(t3.x, 2.0);
        assert_eq!(t3.y, 4.0);
        assert_eq!(t3.z, 6.0);
        assert_eq!(t3.w, 1.0);
    }

    #[test]
    fn can_subtract_points() {
        let p1 = Tuple::point(3.0,2.0, 1.0);
        let p2 = Tuple::point(5.0,6.0, 7.0);
        let t3 = p1.sub(&p2);
        assert_eq!(t3.x, -2.0);
        assert_eq!(t3.y, -4.0);
        assert_eq!(t3.z, -6.0);
        assert_eq!(t3.w, 0.0);
        assert_eq!(p1 - p2, t3);
    }

    #[test]
    fn can_subtract_vector_from_point() {
        let p = Tuple::point(3.0,2.0, 1.0);
        let v = Tuple::vector(5.0,6.0, 7.0);
        let t = p.sub(&v);
        assert_eq!(t.x, -2.0);
        assert_eq!(t.y, -4.0);
        assert_eq!(t.z, -6.0);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn can_subtract_vectors() {
        let v1 = Tuple::vector(3.0,2.0, 1.0);
        let v2 = Tuple::vector(5.0,6.0, 7.0);
        let t3 = v1.sub(&v2);
        assert_eq!(t3.x, -2.0);
        assert_eq!(t3.y, -4.0);
        assert_eq!(t3.z, -6.0);
        assert_eq!(t3.w, 0.0);
    }

    #[test]
    fn can_negate_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let t2 = t.negate();
        assert_eq!(t2.x, -1.0);
        assert_eq!(t2.y, 2.0);
        assert_eq!(t2.z, -3.0);
        assert_eq!(t2.w, 4.0);
        assert_eq!(-t, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn can_multiply_tuple_by_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, 1.0);
        let t2 = t.multiply(3.5);
        assert_eq!(t2.x, 3.5);
        assert_eq!(t2.y, -7.0);
        assert_eq!(t2.z, 10.5);
        assert_eq!(t2.w, 1.0);
        assert_eq!(t * 3.5, t2);
    }

    #[test]
    fn can_multiply_tuple_by_fraction() {
        let t = Tuple::new(1.0, -2.0, 3.0, 1.0);
        let t2 = t.multiply(0.5);
        assert_eq!(t2.x, 0.5);
        assert_eq!(t2.y, -1.0);
        assert_eq!(t2.z, 1.5);
        assert_eq!(t2.w, 1.0);
        assert_eq!(t * 0.5, t2);
    }

    #[test]
    fn can_divide_tuple_by_scalar() {
        let t = Tuple::new(1.0, -2.0, 3.0, 1.0);
        let t2 = t.divide(2.0);
        assert_eq!(t2.x, 0.5);
        assert_eq!(t2.y, -1.0);
        assert_eq!(t2.z, 1.5);
        assert_eq!(t2.w, 1.0);
    }

    #[test]
    fn can_compute_magnitude_of_vector() {
        let v1 = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v1.magnitude(), 1.0);
        let v2 = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v2.magnitude(), 1.0);
        let v3 = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v3.magnitude(), 1.0);
        let v4 = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v4.magnitude(), (14.0 as f64).sqrt());
        let v5 = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v5.magnitude(), (14.0 as f64).sqrt());
    }

    #[test]
    fn can_normalize_vector() {
        let v1 = Tuple::vector(4.0, 0.0, 0.0);
        let v2 = v1.normalize();
        assert_eq!(v2.x, 1.0);
        assert_eq!(v2.y, 0.0);
        assert_eq!(v2.z, 0.0);
        assert_eq!(v2.w, 0.0);
        let v3 = Tuple::vector(1.0, 2.0, 3.0);
        let v4 = v3.normalize();
        assert_eq!(v4.x, 0.2672612419124244);
        assert_eq!(v4.y, 0.5345224838248488);
        assert_eq!(v4.z, 0.8017837257372732);
        assert_eq!(v4.w, 0.0);
        assert_eq!(v4.magnitude(), 1.0);
    }

    #[test]
    fn can_compute_dot_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn can_compute_cross_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x, -1.0);
        assert_eq!(v3.y, 2.0);
        assert_eq!(v3.z, -1.0);
        assert_eq!(v3.w, 0.0);
        let v4 = v2.cross(&v1);
        assert_eq!(v4.x, 1.0);
        assert_eq!(v4.y, -2.0);
        assert_eq!(v4.z, 1.0);
        assert_eq!(v4.w, 0.0);
    }
}