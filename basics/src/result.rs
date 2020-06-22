use std::io;

// Error Handling: Result
fn main() {
    let arr: &[u8] = &[6, 1, 2, 0, 1, 2, 3, 4, 5, 6];

    println!("{:?}", find(&|x| { x / 3 == 2 }, arr));
    println!("{:?}", find(&|x| { x == 9 }, arr));
    println!("{:?}", find(&|x| { x == 6 }, arr));

    let s = match get_weather() {
        Ok(v) => format!("bool : {}", v),
        Err(e) => format!("err : {}", e),
    };
    println!("{}", s);
}

fn find(f: &dyn Fn(u8) -> bool, p: &[u8]) -> Option<u8> {
    match find_index(f, p) {
        Some(n) => Some(p[n]),
        _ => None,
    }
}

fn find_index(k: &dyn Fn(u8) -> bool, p: &[u8]) -> Option<usize> {
    for n in 0..p.len() {
        if k(p[n]) {
            return Some(n);
        }
    }
    None
}

/*

-- | /O(n)/ The 'find' function takes a predicate and a ByteString,
-- and returns the first element in matching the predicate, or 'Nothing'
-- if there is no such element.
--
-- > find f p = case findIndex f p of Just n -> Just (p ! n) ; _ -> Nothing
--
find :: (Word8 -> Bool) -> ByteString -> Maybe Word8
find f p = case findIndex f p of
                    Just n -> Just (p `unsafeIndex` n)
                    _      -> Nothing
{-# INLINE find #-}

-- | The 'findIndex' function takes a predicate and a 'ByteString' and
-- returns the index of the first element in the ByteString
-- satisfying the predicate.
findIndex :: (Word8 -> Bool) -> ByteString -> Maybe Int
findIndex k (PS x s l) = accursedUnutterablePerformIO $ withForeignPtr x $ \f -> go (f `plusPtr` s) 0
  where
    go !ptr !n | n >= l    = return Nothing
               | otherwise = do w <- peek ptr
                                if k w
                                  then return (Just n)
                                  else go (ptr `plusPtr` 1) (n+1)
{-# INLINE findIndex #-}

*/

fn get_weather() -> Result<bool, io::Error> {
    Ok(true)
}
