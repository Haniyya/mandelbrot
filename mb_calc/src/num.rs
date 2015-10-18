mod mb_calc {
    mod num { 
        struct Complex {
            real: f64,
            imaginary: f64,
        }

        impl Complex {
            fn new(real: f64, imaginary: f64) -> Complex {
                Complex {
                    real: real,
                    imaginary: imaginary,
                }
            }
            fn abs(&self) -> f64 {
                let real = self.real * self.real;
                let imaginary = self.imaginary * self.imaginary;

                let sum = real + imaginary;

                let result = sum.sqrt();
                return result;
            }
        }

        #[test]
        fn new_gives_complex() {
            let c = Complex::new(2.0, 2.0 );
            assert_eq!(2.0 ,c.real);
            assert_eq!(2.0 ,c.imaginary);
        }

        #[test]
        fn abs_gives_absolute() {
            let res = 2.0 as f64;
            let c = Complex::new(1.0, 1.0);
            assert_eq!(res.sqrt(), c.abs());
        }
    }
}
