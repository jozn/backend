use rand::Rng;

type Result<T> = std::result::Result<T,SharedIdErr>;

#[derive(PartialEq, Eq,Debug)]
enum SharedIdErr {
    SharedIdRange,
    LocalIdRange,
    VersionUnsupported,
}

#[derive(PartialOrd, PartialEq, Eq, Debug)]
struct ShardId {
    shard_id: u16,
    local_id: u64,
    version: u8,
}

impl ShardId {

    pub fn from_compact(id: u64) -> Result<Self> {
        let ver = id & 0x01;
        if ver != 0 {
            return Err(SharedIdErr::VersionUnsupported)
        }
        assert_eq!(ver, 0); // Only compact version is now supported
        let local_id = (id >> 1) & 0x1ffffffffff;
        let shard_id = ((id >> 41 + 1) & 0x7ff) as u16;

        if shard_id == 0 {
            return Err(SharedIdErr::SharedIdRange)
        }

        if local_id == 0 {
            return Err(SharedIdErr::LocalIdRange)
        }

        Ok(ShardId {
            shard_id,
            local_id,
            version: 0
        })
    }

    pub fn new(shard_id: u16, local_id: u64) -> Result<Self> {
        if shard_id == 0 || shard_id >= (1<<11){
            return Err(SharedIdErr::SharedIdRange)
        }

        if local_id == 0 || local_id >= (1<<41) {
            return Err(SharedIdErr::LocalIdRange)
        }

        Ok(ShardId{
            shard_id,
            local_id,
            version: 0,
        })
    }

    // compact version returns an u64 which is less than 2^53-1 (the first 53 insignificant bits).
    // Tht primary reason for compact design is JavaScrip environment as it does not support larger
    //  integers.
    // bits structure: | 11 bits 0 + 11 bits shared id + 41 bits local id + 1 bit version always 0 |
    // shared id range: 1..2048
    // local id max: ~2.199 Trillion
    pub fn serialize_compact(&self) -> u64 {
        let mut  r = 0;
        // r |= self.version as u64;
        r = 0u64; // version is 0 in compact
        r |= (self.local_id << 1);
        r |= ((self.shard_id as u64) << (1 + 41));

        r
    }

    // This is old version of serialization, it uses 63 bits, for this reason it can not be directely
    //  be used in JavaScript as int.  For now we use compact version in future if we face problems
    //  we can support both compact and 63 bits at the same time.
    fn __ser_63(&self) -> u64 {
        let mut  r = 0;
        r |= self.version as u64;
        r |= (self.local_id << 2);
        r |= ( (self.shard_id as u64 ) << (2 + 47));

        r
    }
}

fn main() {
    let sh1 = ShardId{
        shard_id: 1,
        local_id: 1,
        version: 1
    };

    let sh2 = ShardId{
        shard_id: 1<< (14-1),
        local_id: 1<< (47-1),
        version: ( 1<< (2-1))
    };

    println!("{:064b}, {}", sh1.__ser_63(), sh1.__ser_63());
    println!("{:064b} {}", sh2.__ser_63(), sh2.__ser_63());


    let mut  sh1 = ShardId{
        shard_id: 135,
        local_id: 3436453,
        version: 0
    };
    println!("{:064b}, {}", sh1.__ser_63(), sh1.__ser_63());

    for i in 0..1000 {
        // sh1.local_id +=1;
        sh1.shard_id +=1;
        println!("{:064b}, {}", sh1.__ser_63(), sh1.__ser_63());
    }

    for i in 0..1000 {
        let mut  sh1 = ShardId{
            shard_id: rand::thread_rng().gen_range(0..2000),
            local_id: rand::thread_rng().gen_range(1..(1<<40)),
            version: 0
        };
        // println!("{:064b}, {}", sh1.ser(), sh1.ser());
        println!(">>{:064b}, {}", sh1.serialize_compact(), sh1.serialize_compact());
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_shared_id_pass() {
        let arr = vec![
            (1,1),
            (124,u64::pow(2, 38)),
            (2047,u64::pow(2, 41) -1 ),
            (100,239723),

        ];

        for t in arr {
            let sh1 = ShardId::new(t.0, t.1).unwrap();
            let sh_id = sh1.serialize_compact();
            let sh2 = ShardId::from_compact(sh_id).unwrap();

            println!("{} > {}", t.1, sh_id);

            assert_eq!(sh1, sh2);
        }
    }

    #[test]
    fn test_shared_id_errors() {
        let arr = vec![
            (0,0),
            (0,1),
            (1,0),
            (100,u64::pow(2, 41)),
            (100,u64::pow(2, 45)),
            (2048,100),
            (3000,100),
            (5000,u64::pow(2,42)),
        ];

        for t in arr {
            println!("{}", t.1);
            let sh1 = ShardId::new(t.0, t.1);
            assert!(sh1.is_err());
        }
    }

    #[test]
    fn test_shared_id_version() {
        let arr = vec![
            4398046511106 + 1,
            54590752314 + 1,
            9007199254740 + 1
        ];

        for t in arr {
            let res = ShardId::from_compact(t);
            assert!(res.is_err());
            assert_eq!(res.err().unwrap(), SharedIdErr::VersionUnsupported);
        }
    }

    #[test]
    fn test_shared_id_compact_pass() {
        let arr = vec![ // (sharedId, local_id)
            (1,1),
            (124,u64::pow(2, 38)),
            (2047,u64::pow(2, 41) -1 ),
            (100,239723),

        ];

        for t in arr {
           let num = ((t.0 << 42) | (t.1 << 1) | 0);
            let id = ShardId::from_compact(num).unwrap();
            assert_eq!(id.shard_id, t.0 as u16);
            assert_eq!(id.local_id, t.1);
            assert_eq!(id.serialize_compact(), num);
        }
    }

    #[test]
    fn test_shared_id_from_compact_error() {
        let arr = vec![
            (1,SharedIdErr::VersionUnsupported),
            ( ((11 << 42) | (12 << 1) | 1) ,SharedIdErr::VersionUnsupported),
            ( ((0 << 42) | (12 << 1) | 0) ,SharedIdErr::SharedIdRange),
            ( ((11 << 42) | (0 << 1) | 0) ,SharedIdErr::LocalIdRange),
        ];

        for t in arr {
            // println!("{}", t.1);
            let sh1 = ShardId::from_compact(t.0);
            assert!(sh1.is_err());
            assert_eq!(sh1.err().unwrap(), t.1);
        }
    }
    #[test]
    fn play_shared_id(){
        let sh1 = ShardId{
            shard_id: 1,
            local_id: 1,
            version: 0
        };

        let sh_id = sh1.serialize_compact();
        let sh2 = ShardId::from_compact(sh_id).unwrap();

        assert_eq!(sh1, sh2);

        println!("{:?}", sh1);
        println!("{:?}", sh2);
        println!("{:?}", sh_id);
    }
}