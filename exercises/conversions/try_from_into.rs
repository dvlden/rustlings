use std::convert::{TryInto, TryFrom};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

const RGB_MIN: i16 = 0;
const RGB_MAX: i16 = 255;

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        if tuple.0 < RGB_MIN || tuple.0 > RGB_MAX {
            return Err("First tuple is not within valid RGB range!".to_string());
        }

        if tuple.1 < RGB_MIN || tuple.1 > RGB_MAX {
            return Err("Second tuple is not within valid RGB range!".to_string());
        }

        if tuple.2 < RGB_MIN || tuple.2 > RGB_MAX {
            return Err("Third tuple is not within valid RGB range!".to_string());
        }

        Ok(Color {
            red: tuple.0 as u8,
            green: tuple.1 as u8,
            blue: tuple.2 as u8
        })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        for item in &arr {
            if item < &RGB_MIN || item > &RGB_MAX {
               return Err("An item is not within valid RGB range!".to_string());
            }
        }

        Ok(Color {
            red: arr[0] as u8,
            green: arr[1] as u8,
            blue: arr[2] as u8
        })
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err("A slice exceeds the RGB length!".to_string());
        }

        for s in slice.iter() {
            if s < &RGB_MIN || s > &RGB_MAX {
               return Err("An slice exceeds the RGB range!".to_string());
            }
        }

        Ok(Color {
            red: slice[0] as u8,
            green: slice[1] as u8,
            blue: slice[2] as u8
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_positive() {
        let _ = Color::try_from((256, 1000, 10000)).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tuple_out_of_range_negative() {
        let _ = Color::try_from((-1, -10, -256)).unwrap();
    }

    #[test]
    fn test_tuple_correct() {
        let c: Color = (183, 65, 14).try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_array_out_of_range_positive() {
        let _: Color = [1000, 10000, 256].try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_array_out_of_range_negative() {
        let _: Color = [-10, -256, -1].try_into().unwrap();
    }

    #[test]
    fn test_array_correct() {
        let c: Color = [183, 65, 14].try_into().unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        let _ = Color::try_from(&arr[..]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        let _ = Color::try_from(&arr[..]).unwrap();
    }

    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c = Color::try_from(&v[..]).unwrap();
        assert_eq!(c.red, 183);
        assert_eq!(c.green, 65);
        assert_eq!(c.blue, 14);
    }

    #[test]
    #[should_panic]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        let _ = Color::try_from(&v[..]).unwrap();
    }
}
