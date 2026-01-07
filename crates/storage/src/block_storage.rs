use latte_chain::storage::BlockStorage;
use latte_chain::storage_error::StorageError;
use latte_codec::codec::Codec;
use latte_types::block::Block;
use rocksdb::{DB, Options, Error};

struct RocksDbBlockStorage<C: Codec> {
    db: rocksdb::DB,
    codec: C,
}

impl<C: Codec> RocksDbBlockStorage<C> {
    fn new(path: &str, c: C) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db_result = DB::open(&opts, "./block_db");
        match db_result {
            Ok(db) => Self { db, codec: c },
            Err(e) => {
                panic!("Failed to open block db: {:?}", e);
            }
        }
    }
}

impl<C: Codec> BlockStorage for RocksDbBlockStorage<C> {
    fn get_block(
        &self,
        block_hash: latte_primitives::hash::Hash256,
    ) -> Result<Option<Block>, StorageError> {
        let result = self.db.get(block_hash.0);
        match result {
            Ok(Some(value)) => {
                let block: Result<Block, String> = self.codec.decode(&value.to_vec());
                match block {
                    Ok(block) => Ok(Some(block)),
                    Err(e) => Err(StorageError::BlockGetFailed(e.to_string())),
                }
            }
            Ok(None) => Err(StorageError::BlockNotFound),
            Err(e) => Err(StorageError::Db(e.to_string()).into()),
        }
    }

    fn put_block(&self, block: &Block) -> Result<(), StorageError> {
        let hash_byte;
        let block_byte;

        match self.codec.encode(&block.header.number) {
            Ok(data) => {hash_byte = data;}
            Err(e) => return Err(StorageError::Db(e).into()),
        }

        match self.codec.encode(&block) {
            Ok(data) => {block_byte = data;}
            Err(e) => return Err(StorageError::Db(e).into()),
        }

        let result = self.db.put(hash_byte,block_byte);
        match result {
            Ok(_) => {Ok(())}
            Err(e) => Err(StorageError::Db(e.to_string()).into()),
        }

    }
}
