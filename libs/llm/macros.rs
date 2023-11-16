/// Sigma `Σ` macro
/// 
/// This is basically a for loop that sums up the values of the expression
#[macro_export(local_inner_macros)]
macro_rules! Σ {
    ($finish:expr => $( $tt:tt )*) => (
        $crate::Σ!(0, $finish, 1 => $( $tt )*)
    );

    ($start:expr, $finish:expr, $step:literal => { $( $tt:tt )* }) => {{
        let mut sum = T::from(0.0);
        let mut i = $start;
        while i < $finish {
            sum += {
                $( $tt )*
            };
            i += $step;
        }
        sum
    }};
}

pub use Σ;
