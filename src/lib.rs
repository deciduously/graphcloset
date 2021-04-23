//! The graphcloset library toplevel.

use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

/// An (x,y) coordinate pair
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(t: (i32, i32)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref POINT_RE: Regex = Regex::new(r"\((?P<x>\d), ?(?P<y>\d)\)").unwrap();
        }

        // assert there is a match
        if !POINT_RE.is_match(s) {
            return Err(anyhow!("Invalid point string {}", s));
        }

        // Unwraps are all safe, the regex is already known to match
        let captures = POINT_RE.captures(s).unwrap();
        Ok(Self {
            x: captures.name("x").unwrap().as_str().parse::<i32>().unwrap(),
            y: captures.name("y").unwrap().as_str().parse::<i32>().unwrap(),
        })
    }
}

/// The data needed to render a graph.
#[derive(Debug, Default, PartialEq)]
struct Graph {
    points: Vec<Point>,
}

impl FromStr for Graph {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        let point_strs = s.split(' ');
        for point in point_strs {
            points.push(Point::from_str(point).unwrap());
        }
        Ok(Self { points })
    }
}

/// Take a series of points in string for and produce a plot in string form.
pub fn plot(input: &str) -> String {
    format!("{:?}", Graph::from_str(input))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_point_from_str() {
        let input = "(1,2)";
        let input2 = "(1, 2)";
        let point = Point::from_str(input).unwrap();
        let point2 = Point::from_str(input).unwrap();
        assert_eq!(point, point2);
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }
    #[test]
    #[should_panic]
    fn test_point_from_str_invalid() {
        let bad_input = "(1,)2";
        let _ = Point::from_str(bad_input).unwrap();
        unreachable!()
    }
    #[test]
    fn test_graph_from_str() {
        let default = "(4,5) (1,2) (7,8)";
        let graph = Graph::from_str(default).unwrap();
        assert_eq!(
            graph,
            Graph {
                points: vec![
                    Point::from((4, 5)),
                    Point::from((1, 2)),
                    Point::from((7, 8))
                ]
            }
        );
    }
}
