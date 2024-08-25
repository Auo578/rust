// 课后习题
// 修复call函数的错误
// 当b为None时，按默认值1 
fn call(a: i32,b:i32) -> Result<f64,String> {
    let r= divide(a, b).unwrap_or(0.1);
    let s= sqrt(r).map_err(|e| format!("{:?}",e))?;

    Ok(s)
}
fn divide(a: i32, b:i32)-> Option<f64>{
    if b != 0{
        Some(a as f64 / b as f64)
    } else {
        None
    }
}
#[derive(Debug)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn sqrt(x:f64)-> Result<f64, MathError>{
    if x< 0.0{
        Err(MathError::NegativeSquareRoot)
        } else {
        Ok (x.sqrt())
        }
}

fn main() {
    // 调用 `call` 函数并处理结果
    match call(10, 2) {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

