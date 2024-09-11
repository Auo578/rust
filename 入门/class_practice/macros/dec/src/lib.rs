#[macro_export]
macro_rules! repeat {
    ( $x:expr , $count:expr  ) => {
        {
            $x.repeat($count)
        }
    };
}

#[macro_export]
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    };
}

#[macro_export]
macro_rules! max_value{
    ( $x:expr  ) => {
        $x
    };
    ( $x:expr, $($rest:expr),*) => {
        {
            let max_rest = max_value!($($rest),*);
            if $x > max_rest {
                $x
            } else {
                max_rest
            }
        }
    };
}
