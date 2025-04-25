#![doc = include_str!("../doc/dimensional.md")]
use num::Rational32;
use std::clone::Clone;
use std::fmt;
use std::ops;

#[derive(Clone, Debug)]
pub enum Dimension {
    PowerLaw(i32, i32, i32, i32, i32, i32, i32),
    PowerLawIA([i32; 7]),
    // allow rational exponents for each dimension, temporaryly, only for calculation
    PowerLawR(
        Rational32,
        Rational32,
        Rational32,
        Rational32,
        Rational32,
        Rational32,
        Rational32,
    ),
    // allow rational exponents for each dimension, temporaryly, only for calculation
    PowerLawRA([Rational32; 7]),
    Product {
        left: Box<Dimension>,
        right: Box<Dimension>,
    },
    Division {
        left: Box<Dimension>,
        right: Box<Dimension>,
    },
    PowerI {
        original: Box<Dimension>,
        power: i32,
    },
    PowerR {
        original: Box<Dimension>,
        power: Rational32,
    },
}

pub const DIMENSIONAL_LABELS: [&str; 7] = ["L", "M", "T", "I", "Θ", "N", "J"];

// major dimensions
pub const DIMENSIONLESS: Dimension = Dimension::PowerLaw(0, 0, 0, 0, 0, 0, 0);
pub const LENGTH: Dimension = Dimension::PowerLaw(1, 0, 0, 0, 0, 0, 0);
pub const MASS: Dimension = Dimension::PowerLaw(0, 1, 0, 0, 0, 0, 0);
pub const TIME: Dimension = Dimension::PowerLaw(0, 0, 1, 0, 0, 0, 0);
pub const ELECTRIC_CURRENT: Dimension = Dimension::PowerLaw(0, 0, 0, 1, 0, 0, 0);
pub const TEMPERATURE: Dimension = Dimension::PowerLaw(0, 0, 0, 0, 1, 0, 0);
pub const AMOUNT_OF_SUBSTANCE: Dimension = Dimension::PowerLaw(0, 0, 0, 0, 0, 1, 0);
pub const LUMINOUS_INTENSITY: Dimension = Dimension::PowerLaw(0, 0, 0, 0, 0, 0, 1);

// derived dimensions
pub const AREA: Dimension = Dimension::PowerLaw(2, 0, 0, 0, 0, 0, 0);
pub const VOLUME: Dimension = Dimension::PowerLaw(3, 0, 0, 0, 0, 0, 0);
pub const FREQUENCY: Dimension = Dimension::PowerLaw(0, 0, -1, 0, 0, 0, 0);
pub const VELOCITY: Dimension = Dimension::PowerLaw(1, 0, -1, 0, 0, 0, 0);
pub const ACCELERATION: Dimension = Dimension::PowerLaw(1, 0, -2, 0, 0, 0, 0);
pub const FORCE: Dimension = Dimension::PowerLaw(1, 1, -2, 0, 0, 0, 0);
pub const ENERGY: Dimension = Dimension::PowerLaw(2, 1, -2, 0, 0, 0, 0);
pub const POWER: Dimension = Dimension::PowerLaw(2, 1, -3, 0, 0, 0, 0);
pub const DENSITY: Dimension = Dimension::PowerLaw(-3, 1, 0, 0, 0, 0, 0);
pub const PRESSURE: Dimension = Dimension::PowerLaw(-1, 1, -2, 0, 0, 0, 0);
pub const ELECTRIC_CHARGE: Dimension = Dimension::PowerLaw(0, 0, 1, 1, 0, 0, 0);
pub const ELECTRIC_POTENTIAL: Dimension = Dimension::PowerLaw(2, 1, -2, -1, 0, 0, 0);
pub const RESISTANCE: Dimension = Dimension::PowerLaw(2, 1, -2, -2, 0, 0, 0);
pub const CAPACITANCE: Dimension = Dimension::PowerLaw(-2, -1, 3, 2, 0, 0, 0);
pub const INDUCTANCE: Dimension = Dimension::PowerLaw(2, 1, -2, -2, 0, 0, 0);
pub const FLUX: Dimension = Dimension::PowerLaw(2, 1, -1, -1, 0, 0, 0);
pub const FLUX_DENSITY: Dimension = Dimension::PowerLaw(0, 1, -1, -1, 0, 0, 0);
pub const MAGNETIC_FIELD: Dimension = Dimension::PowerLaw(0, 1, -1, -1, 0, 0, 0);
pub const MAGNETIC_FLUX: Dimension = Dimension::PowerLaw(2, 1, -1, -1, 0, 0, 0);
pub const MAGNETIC_FLUX_DENSITY: Dimension = Dimension::PowerLaw(1, 1, -1, -1, 0, 0, 0);

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_dimension_string())
    }
}

impl PartialEq for Dimension {
    fn eq(&self, other: &Self) -> bool {
        let se = self.get_exponents();
        let oe = other.get_exponents();
        se.iter().zip(oe.iter()).all(|(a, b)| a == b)
    }
}

pub trait DimensionTrait {
    fn standard_form(&self) -> Dimension;
}

impl DimensionTrait for Dimension {
    fn standard_form(&self) -> Dimension {
        let exponents = self.get_exponents();
        if exponents.iter().map(|e| e.denom()).all(|d| d == &1) {
            return Dimension::PowerLaw(
                *exponents[0].numer(),
                *exponents[1].numer(),
                *exponents[2].numer(),
                *exponents[3].numer(),
                *exponents[4].numer(),
                *exponents[5].numer(),
                *exponents[6].numer(),
            );
        }
        panic!("Dimension is not in standard form");
    }
}

impl Dimension {
    fn to_dimension_string(&self) -> String {
        let exponents = self.get_exponents();
        let mut result = String::new();
        for (i, exponent) in exponents.iter().enumerate() {
            if *exponent != Rational32::from_integer(0) {
                if !result.is_empty() {
                    result.push_str("");
                }
                result.push_str(DIMENSIONAL_LABELS[i]);
                if *exponent != Rational32::from_integer(1) {
                    result.push('^');
                    if exponent.denom() == &1 {
                        result.push_str(&exponent.numer().to_string());
                    } else {
                        result.push_str(&format!("{}/{}", exponent.numer(), exponent.denom()));
                    }
                }
            }
        }
        if result.is_empty() {
            result.push_str("-"); // 如果所有维度都是0，显示为1
        }
        result
    }
    // define operator to get exponent of a given dimension
    pub fn get_exponents(&self) -> [Rational32; 7] {
        match self {
            Dimension::PowerLaw(a, b, c, d, e, f, g) => [
                Rational32::from_integer(*a),
                Rational32::from_integer(*b),
                Rational32::from_integer(*c),
                Rational32::from_integer(*d),
                Rational32::from_integer(*e),
                Rational32::from_integer(*f),
                Rational32::from_integer(*g),
            ],
            Dimension::PowerLawIA(a) => [
                Rational32::from_integer(a[0]),
                Rational32::from_integer(a[1]),
                Rational32::from_integer(a[2]),
                Rational32::from_integer(a[3]),
                Rational32::from_integer(a[4]),
                Rational32::from_integer(a[5]),
                Rational32::from_integer(a[6]),
            ],
            Dimension::PowerLawR(a, b, c, d, e, f, g) => [*a, *b, *c, *d, *e, *f, *g],
            Dimension::PowerLawRA(a) => *a,
            Dimension::Product { left, right } => {
                let a_exponents = left.get_exponents();
                let b_exponents = right.get_exponents();
                [
                    a_exponents[0] + b_exponents[0],
                    a_exponents[1] + b_exponents[1],
                    a_exponents[2] + b_exponents[2],
                    a_exponents[3] + b_exponents[3],
                    a_exponents[4] + b_exponents[4],
                    a_exponents[5] + b_exponents[5],
                    a_exponents[6] + b_exponents[6],
                ]
            }
            Dimension::Division { left, right } => {
                let ae = left.get_exponents();
                let be = right.get_exponents();
                [
                    ae[0] - be[0],
                    ae[1] - be[1],
                    ae[2] - be[2],
                    ae[3] - be[3],
                    ae[4] - be[4],
                    ae[5] - be[5],
                    ae[6] - be[6],
                ]
            }
            Dimension::PowerI { original, power } => {
                let ae = original.get_exponents();
                ae.map(|e| e * Rational32::from_integer(*power))
            }
            Dimension::PowerR { original, power } => {
                let ae = original.get_exponents();
                ae.map(|e| e * *power)
            }
        }
    }

    /// 获取指定维度的指数
    pub fn get_exponent(&self, index: usize) -> Rational32 {
        if index >= 7 {
            panic!("Index out of bounds: {}", index);
        }
        self.get_exponents()[index]
    }

    /// 设置指定维度的指数
    pub fn with_exponent(self, index: usize, exponent: Rational32) -> Self {
        if index >= 7 {
            panic!("Index out of bounds: {}", index);
        }
        let mut exponents = self.get_exponents();
        exponents[index] = exponent;
        Dimension::PowerLawRA(exponents)
    }

    /// 检查维度是否为零（所有指数都为零）
    pub fn is_zero(&self) -> bool {
        self.get_exponents()
            .iter()
            .all(|&e| e == Rational32::from_integer(0))
    }

    /// 检查维度是否为无量纲（所有指数都为零）
    pub fn is_dimensionless(&self) -> bool {
        self.is_zero()
    }

    // /// 检查维度是否相等（所有指数都相等）
    // fn equals(&self, other: &Self) -> bool {
    //     self.get_exponents().iter()
    //         .zip(other.get_exponents().iter())
    //         .all(|(a, b)| a == b)
    // }

    /// 获取维度的幂
    pub fn pow(self, power: Rational32) -> Self {
        Dimension::PowerR {
            original: Box::new(self.standard_form()),
            power,
        }
    }

    pub fn powi(self, power: i32) -> Self {
        self.pow(Rational32::from_integer(power))
    }

    /// 获取维度的倒数
    pub fn reciprocal(self) -> Self {
        self.pow(Rational32::from_integer(-1))
    }

    /// 获取维度的平方
    pub fn square(self) -> Self {
        self.pow(Rational32::from_integer(2))
    }

    /// 获取维度的平方根
    pub fn sqrt(self) -> Self {
        self.pow(Rational32::new(1, 2))
    }
}

impl ops::Add for Dimension {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // check eq of self, rhs, panic if not eq
        if self == rhs {
            self.standard_form()
        } else {
            panic!("{} != {}, no add operation allowed.", self, rhs)
        }
    }
}

impl ops::Sub for Dimension {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // check eq of self, rhs, panic if not eq
        if self == rhs {
            self.standard_form()
        } else {
            panic!("{} != {}, no sub operation allowed.", self, rhs)
        }
    }
}

impl ops::Mul for Dimension {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Dimension::Product {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl ops::Div for Dimension {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Dimension::Division {
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl ops::Index<usize> for Dimension {
    type Output = Rational32;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= 7 {
            panic!("Index out of bounds: {}", index);
        }
        let exponents = self.get_exponents();
        Box::leak(Box::new(exponents[index]))
    }
}
