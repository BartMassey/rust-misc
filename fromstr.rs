// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

fn main() {
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32
    }

    #[derive(Debug)]
    enum PointError {
        CoordDescErr(std::num::ParseIntError),
        CoordCountErr(usize)
    }

    impl std::fmt::Display for PointError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                &PointError::CoordDescErr(ref e) =>
                    write!(f, "coord parse error: {}", e),
                &PointError::CoordCountErr(n) =>
                    write!(f, "expected 2 coords, got {}", n),
            }
        }
    }

    impl std::error::Error for PointError {
        fn description(&self) -> &str {
            "point error"
        }
    }

    impl FromStr for Point {
        type Err = PointError;

        fn from_str(s: &str) -> Result<Self, PointError> {
            let coords = s
                // XXX This will accept ((1,2)) etc.
                .trim_matches(|p| p == '(' || p == ')' )
                .split(",")
                .map(|s| s
                     .trim()
                     .parse()
                     .map_err(|e| PointError::CoordDescErr(e)))
                .collect::<Result<Vec<i32>, PointError>>()?;

            if coords.len() != 2 {
                return Err(PointError::CoordCountErr(coords.len()));
            }

            Ok(Point { x: coords[0], y: coords[1] })
        }
    }

    
    let p = Point::from_str(&std::env::args().nth(1).unwrap());
    println!("{:?}", p);
}
