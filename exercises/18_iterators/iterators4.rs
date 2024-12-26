fn factorial_for(num: u64) -> u64 {
    let mut res = 1;
    for i in 2..=num {
        res *= i;
    }
    res
}

fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion

    // Case num==0: The iterator 2..=0 is empty
    //              -> The initial value of `fold` is returned which is 1.
    // Case num==1: The iterator 2..=1 is also empty
    //              -> The initial value 1 is returned.
    // Case num==2: The iterator 2..=2 contains one element
    //              -> The initial value 1 is multiplied by 2 and the result
    //                 is returned.
    // Case num==3: The iterator 2..=3 contains 2 elements
    //              -> 1 * 2 is calculated, then the result 2 is multiplied by
    //                 the second element 3 so the result 6 is returned.
    // And so on…
    (2..=num).fold(1, |acc, x| acc * x)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
