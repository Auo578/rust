fn main() {
    // 无限制条件的 Fibonacci 迭代器
    let fib = Fibonacci::new(None);

    // 使用 take 方法获取前 100 个斐波那契数
    let first_10: Vec<u64> = fib.take(100).collect();
    println!("前 10 个斐波那契数: {:?}", first_10);

    // 限制3000的 Fibonacci 迭代器
    let fib_with_limit = Fibonacci::new(Some(3000));

    // 将生成的斐波那契数转换为 Vec<u64>
    let fib_vec = fib_with_limit.into_vec();
    println!("斐波那契数列（不超过 1000): {:?}", fib_vec);
}




struct Fibonacci {
    current: u64,
    next: u64,
    limit: Option<u64>,
}

impl Fibonacci {
    fn new(limit: Option<u64>) -> Self {
        Fibonacci {
            current: 0,
            next: 1,
            limit,
        }
    }


    fn into_vec(self) -> Vec<u64> {
        self.into_iter()     //take_while(|&x| true) //在这里暂时没有限制条件，可以改成iter_into（）,要获取所有权
        .collect()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(limit) = self.limit {
            if self.current > limit {
                return None;
            }
        }

        //let new_next = self.current + self.next;
        let current_value = self.current;
        let new_next = self.current.checked_add(self.next)?;
        //checked.addd() 方法在溢出时返回 None，而不是引发 panic。
        self.current = self.next;
        self.next = new_next;

        Some(current_value)//不可以用self.current，因为current已经改变了
    }
}
