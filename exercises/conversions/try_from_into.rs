use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

// 1. Tuple implementation (Already completed)
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        if tuple.0 < 0
            || tuple.0 > 255
            || tuple.1 < 0
            || tuple.1 > 255
            || tuple.2 < 0
            || tuple.2 > 255
        {
            return Err(IntoColorError::IntConversion);
        }
        Ok(Color {
            red: tuple.0 as u8,
            green: tuple.1 as u8,
            blue: tuple.2 as u8,
        })
    }
}

// 2. Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        // Since the array length is guaranteed to be 3 by the type [i16; 3],
        // we just pass the elements into the tuple implementation.
        Color::try_from((arr[0], arr[1], arr[2]))
    }
}

// 3. Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        // Check the length first
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }
        // If length is 3, pass elements to the tuple implementation
        Color::try_from((slice[0], slice[1], slice[2]))
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}
