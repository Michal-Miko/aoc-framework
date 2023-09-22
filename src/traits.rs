use itertools::Itertools;
use std::error::Error;
use std::fmt::Display;

use crate::AocSolution;

pub trait Solved {
    fn solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>>;
}

impl<I> Solved for I
where
    I: IntoIterator,
    I::Item: Display,
{
    fn solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>> {
        Ok(self
            .into_iter()
            .map(|element| element.to_string())
            .collect())
    }
}

pub trait TrySolved {
    fn try_solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>>;
}

impl<I, S, E> TrySolved for I
where
    I: IntoIterator<Item = Result<S, E>>,
    S: std::fmt::Display,
    E: Into<Box<dyn Error + Sync + Send>>,
{
    fn try_solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>> {
        self.into_iter()
            .map(|result| match result {
                Ok(element) => Ok(element.to_string()),
                Err(err) => Err(err.into()),
            })
            .try_collect()
    }
}

pub trait UnitSolved {
    fn solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>>;
}

impl<S: Display> UnitSolved for S {
    fn solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>> {
        Ok(vec![self.to_string()])
    }
}

pub trait TryUnitSolved {
    fn try_solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>>;
}

impl<S, E> TryUnitSolved for Result<S, E>
where
    S: Display,
    E: Into<Box<dyn Error + Sync + Send>>,
{
    fn try_solved(self) -> Result<AocSolution, Box<dyn Error + Sync + Send>> {
        match self {
            Ok(unit) => Ok(vec![unit.to_string()]),
            Err(err) => Err(err.into()),
        }
    }
}
