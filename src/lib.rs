#![allow(unused)]

fn str_len(s: &str) -> usize {
    s.len()
}

async fn str_len_async(s: &str) -> usize {
    // do something awaitable ideally...
    s.len()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    #[test]
    fn test_str_len() {
        assert_eq!(str_len("x5ff"), 4);
    }

    #[actix_rt::test]
    async fn test_str_len_async() {
        assert_eq!(str_len_async("x5ff").await, 4);
    }

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_str_len_async_2() {
        assert_eq!(aw!(str_len_async("x5ff")), 4);
    }
}
