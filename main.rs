use std::{
    error,
    fs::{read_to_string, write},
    path::Path,
    result,
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

fn read_file(p: &str) -> TResult<String> {
    return read_to_string(p).map_err(|e| e.into());
}

fn split_numbers(s: &String) -> TResult<Vec<usize>> {
    return s
        .split_whitespace()
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect();
}

fn add_numbers(v: Vec<usize>) -> usize {
    return v.iter().fold(0, |mut sum, &x| {
        sum = sum + x;
        sum
    });
}

fn write_numbers(n: usize, p: &str) -> TResult<()> {
    let path = Path::new(p);
    let res = read_file(&path.display().to_string())?;

    write(path, format!("{}\n{}", res, n))?;
    Ok(())
}

fn main() -> TResult<()> {
    let path = "numbers.txt";
    let res = read_file(&path);

    match res {
        Ok(s) => {
            let vec = split_numbers(&s)?;
            let sum = add_numbers(vec);
            write_numbers(sum, path)?;
        }
        Err(_) => {}
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_test_write() -> TResult<()> {
        write(Path::new("input.txt"), String::from("4\n6"))?;
        Ok(())
    }

    #[test]
    fn test_read_file() {
        let res = read_file("something.txt");
        if let Ok(s) = res {
            assert_eq!(s, "");
        }
    }

    #[test]
    fn test_split_numbers() {
        let res = split_numbers(&String::from("5\n8"));
        assert!(res.is_ok());

        if let Ok(v) = res {
            assert_eq!(v, vec![5, 8])
        }
    }

    #[test]
    fn test_add_numbers() {
        let sum = add_numbers(vec![3, 6]);
        assert!(sum == 9);
    }

    #[test]
    fn test_write_numbers() {
        let res = setup_test_write();
        assert!(res.is_ok());

        let res = write_numbers(10, "input.txt");
        assert!(res.is_ok());

        let res = read_file("input.txt");

        if let Ok(s) = res {
            assert_eq!(s, "4\n6\n10");
        }
    }
}
