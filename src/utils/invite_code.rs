use std::char;

use std::time::{SystemTime, UNIX_EPOCH};

const OFFSET: u64 = 60466176;
const BASE: u64 = 36;
// invite code character = ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789
// static CHARACTER_SET: &'static [u32] = &[51,52,81,65,53,77,90,56,71,83,88,69,73,50,79,66,70,67,80,78,68,85,76,84,82,49,89,54,55,48,74,57,86,75,87,72];
// static CHARACTER_SET: &'static [u32] = &[49,80,55,72,48,74,66,83,89,84,50,73,77,56,68,71,70,57,53,86,54,69,87,85,79,81,67,51,52,88,65,90,78,75,76,82];
// static CHARACTER_SET: &'static [u32] = &[86,72,65,84,48,90,87,55,56,53,83,57,85,88,81,75,73,89,80,70,71,74,52,76,68,82,77,66,69,79,54,50,67,49,51,78];
// static CHARACTER_SET: &'static [u32] = &[86,74,54,65,75,82,51,90,84,50,81,71,67,73,57,85,52,55,89,72,76,83,48,87,53,77,69,66,56,80,49,68,88,78,79,70];
// static CHARACTER_SET: &'static [u32] = &[81,75,90,72,80,70,84,52,48,51,66,71,77,67,55,53,57,87,54,88,89,68,74,85,79,69,73,76,86,78,82,83,50,65,56,49];
// static CHARACTER_SET: &'static [u32] = &[75,86,80,85,50,66,90,65,54,56,89,83,57,84,51,52,73,67,48,88,81,78,87,69,70,55,79,71,77,72,53,82,49,76,74,68];
// static CHARACTER_SET: &'static [u32] = &[71,48,90,65,82,57,66,51,86,69,70,87,74,55,56,88,53,67,81,76,89,73,78,79,80,52,54,75,84,72,49,83,85,50,68,77];
// static CHARACTER_SET: &'static [u32] = &[74,73,80,50,52,83,75,53,70,88,48,85,86,81,49,90,65,56,89,68,66,84,57,51,76,77,67,69,72,54,87,82,55,79,71,78];
// static CHARACTER_SET: &'static [u32] = &[72,88,49,79,66,77,89,75,84,78,70,71,52,69,80,57,67,56,81,87,55,48,68,54,65,73,74,85,50,90,51,83,82,76,53,86];
// static CHARACTER_SET: &'static [u32] = &[56,88,74,82,81,83,78,76,53,79,69,49,65,67,90,66,50,68,89,80,70,71,75,84,55,86,72,52,73,48,85,54,77,87,51,57];
static CHARACTER_SET: &'static [u32] = &[
    87, 67, 54, 90, 50, 82, 69, 55, 56, 88, 71, 68, 65, 72, 78, 89, 79, 74, 57, 48, 52, 66, 86, 53,
    70, 49, 51, 80, 83, 76, 77, 84, 81, 75, 73, 85,
];

pub struct InviteCode {}

impl InviteCode {
    pub fn new(mut id: u64) -> String {
        id += OFFSET;
        let mut invite_code_nums = "".to_string();
        while id > 0 {
            let remain = id % BASE;
            invite_code_nums.push_str(&Self::calc_offset(remain).to_string());
            id /= BASE;
        }
        let timestamp = Self::make_timestamp() % 1_000_000;
        invite_code_nums.push_str(&format!("{:06}", timestamp));
        invite_code_nums
    }

    fn get_id_range() -> (u64, u64) {
        let min = 1;
        let max = BASE.pow(6) - OFFSET;
        (min, max)
    }

    fn calc_offset(remain: u64) -> char {
        char::from_u32(CHARACTER_SET[remain as usize]).unwrap()
    }

    fn make_timestamp() -> u64 {
        let start = SystemTime::now();
        start.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::{InviteCode, OFFSET};

    #[test]
    fn test_invite_code_length() {
        let invite_code = InviteCode::new(1);
        assert_eq!(12, invite_code.len());
        let invite_code = InviteCode::new(2116316159);
        assert_eq!(12, invite_code.len());
        let invite_code = InviteCode::new(2116316160);
        assert_eq!(13, invite_code.len());
    }

    #[test]
    fn test_invite_code_not_same() {
        let invite_code_1 = InviteCode::new(1);
        let invite_code_2 = InviteCode::new(2);
        debug!("{:?} vs {:?}", invite_code_1, invite_code_2);
        assert_ne!(invite_code_1, invite_code_2);
    }

    #[test]
    fn test_make_tiestamp() {
        let ts = InviteCode::make_timestamp();
        let new_ts = InviteCode::make_timestamp();
        assert!(new_ts > ts);
    }

    #[test]
    fn test_invite_code_id_range() {
        let (min, max) = InviteCode::get_id_range();
        assert_eq!(min, 1);
        assert_eq!(36u64.pow(5), OFFSET);
        assert_eq!(36u64.pow(6) - OFFSET, max);
    }
}
