use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::{btree_entry::Reducer, BinarySerialization, PagedWriter};
use crate::{error::Error, tree::key_entry::ValueIndex, ArcBytes};

/// The index stored within [`VersionedTreeRoot::by_id_root`](crate::tree::VersionedTreeRoot::by_id_root).
#[derive(Clone, Debug)]
pub struct VersionedByIdIndex<EmbeddedIndex: super::EmbeddedIndex> {
    /// The unique sequence id generated when writing the value to the file.
    pub sequence_id: u64,
    /// The size of the value stored on disk.
    pub value_length: u32,
    /// The position of the value on disk.
    pub position: u64,
    /// The embedded index.
    pub embedded: EmbeddedIndex,
}

impl<EmbeddedIndex> BinarySerialization for VersionedByIdIndex<EmbeddedIndex>
where
    EmbeddedIndex: super::EmbeddedIndex,
{
    fn serialize_to(
        &mut self,
        writer: &mut Vec<u8>,
        _paged_writer: &mut PagedWriter<'_>,
    ) -> Result<usize, Error> {
        writer.write_u64::<BigEndian>(self.sequence_id)?;
        writer.write_u32::<BigEndian>(self.value_length)?;
        writer.write_u64::<BigEndian>(self.position)?;
        Ok(20 + self.embedded.serialize_to(writer)?)
    }

    fn deserialize_from(
        reader: &mut ArcBytes<'_>,
        _current_order: Option<usize>,
    ) -> Result<Self, Error> {
        let sequence_id = reader.read_u64::<BigEndian>()?;
        let value_length = reader.read_u32::<BigEndian>()?;
        let position = reader.read_u64::<BigEndian>()?;
        Ok(Self {
            sequence_id,
            value_length,
            position,
            embedded: EmbeddedIndex::deserialize_from(reader)?,
        })
    }
}

impl<EmbeddedIndex: super::EmbeddedIndex> ValueIndex for VersionedByIdIndex<EmbeddedIndex> {
    fn position(&self) -> u64 {
        self.position
    }
}

/// The index stored within [`UnversionedTreeRoot::by_id_root`](crate::tree::UnversionedTreeRoot::by_id_root).
#[derive(Clone, Debug)]
pub struct UnversionedByIdIndex<EmbeddedIndex: super::EmbeddedIndex> {
    /// The size of the value stored on disk.
    pub value_length: u32,
    /// The position of the value on disk.
    pub position: u64,
    /// The embedded index.
    pub embedded: EmbeddedIndex,
}

impl<EmbeddedIndex> BinarySerialization for UnversionedByIdIndex<EmbeddedIndex>
where
    EmbeddedIndex: super::EmbeddedIndex,
{
    fn serialize_to(
        &mut self,
        writer: &mut Vec<u8>,
        _paged_writer: &mut PagedWriter<'_>,
    ) -> Result<usize, Error> {
        writer.write_u32::<BigEndian>(self.value_length)?;
        writer.write_u64::<BigEndian>(self.position)?;
        Ok(12 + self.embedded.serialize_to(writer)?)
    }

    fn deserialize_from(
        reader: &mut ArcBytes<'_>,
        _current_order: Option<usize>,
    ) -> Result<Self, Error> {
        let value_length = reader.read_u32::<BigEndian>()?;
        let position = reader.read_u64::<BigEndian>()?;
        Ok(Self {
            value_length,
            position,
            embedded: EmbeddedIndex::deserialize_from(reader)?,
        })
    }
}

impl<EmbeddedIndex: super::EmbeddedIndex> ValueIndex for UnversionedByIdIndex<EmbeddedIndex> {
    fn position(&self) -> u64 {
        self.position
    }
}

/// The reduced index of both [`VersionedByIdIndex`] and [`UnversionedByIdIndex`]
#[derive(Clone, Debug)]
pub struct ByIdStats<EmbeddedStats> {
    /// The number of keys that have values stored within them.
    pub alive_keys: u64,
    /// The number of keys that no longer have values stored within them.
    pub deleted_keys: u64,
    /// The total number of bytes stored on disk associated with currently-alive values.
    pub total_indexed_bytes: u64,
    /// The embedded statistics.
    pub embedded: EmbeddedStats,
}

impl<EmbeddedStats> ByIdStats<EmbeddedStats> {
    /// Returns the total number of keys regardless of whether data is stored within them.
    #[must_use]
    pub const fn total_keys(&self) -> u64 {
        self.alive_keys + self.deleted_keys
    }
}

impl<EmbeddedStats> BinarySerialization for ByIdStats<EmbeddedStats>
where
    EmbeddedStats: super::Serializable,
{
    fn serialize_to(
        &mut self,
        writer: &mut Vec<u8>,
        _paged_writer: &mut PagedWriter<'_>,
    ) -> Result<usize, Error> {
        writer.write_u64::<BigEndian>(self.alive_keys)?;
        writer.write_u64::<BigEndian>(self.deleted_keys)?;
        writer.write_u64::<BigEndian>(self.total_indexed_bytes)?;
        Ok(24 + self.embedded.serialize_to(writer)?)
    }

    fn deserialize_from(
        reader: &mut ArcBytes<'_>,
        _current_order: Option<usize>,
    ) -> Result<Self, Error> {
        let alive_keys = reader.read_u64::<BigEndian>()?;
        let deleted_keys = reader.read_u64::<BigEndian>()?;
        let total_indexed_bytes = reader.read_u64::<BigEndian>()?;
        Ok(Self {
            alive_keys,
            deleted_keys,
            total_indexed_bytes,
            embedded: EmbeddedStats::deserialize_from(reader)?,
        })
    }
}

impl<EmbeddedIndex, EmbeddedStats> Reducer<VersionedByIdIndex<EmbeddedIndex>>
    for ByIdStats<EmbeddedStats>
where
    EmbeddedIndex: super::EmbeddedIndex,
    EmbeddedStats: Reducer<EmbeddedIndex>,
{
    fn reduce<'a, Indexes, IndexesIter>(indexes: Indexes) -> Self
    where
        EmbeddedIndex: 'a,
        Indexes: IntoIterator<Item = &'a VersionedByIdIndex<EmbeddedIndex>, IntoIter = IndexesIter>
            + ExactSizeIterator,
        IndexesIter:
            Iterator<Item = &'a VersionedByIdIndex<EmbeddedIndex>> + ExactSizeIterator + Clone,
    {
        reduce(indexes)
    }

    fn rereduce<
        'a,
        ReducedIndexes: IntoIterator<Item = &'a Self, IntoIter = ReducedIndexesIter> + ExactSizeIterator,
        ReducedIndexesIter: Iterator<Item = &'a Self> + ExactSizeIterator + Clone,
    >(
        values: ReducedIndexes,
    ) -> Self
    where
        Self: 'a,
    {
        rereduce(values)
    }
}

impl<EmbeddedIndex, EmbeddedStats> Reducer<UnversionedByIdIndex<EmbeddedIndex>>
    for ByIdStats<EmbeddedStats>
where
    EmbeddedIndex: super::EmbeddedIndex,
    EmbeddedStats: Reducer<EmbeddedIndex>,
{
    fn reduce<'a, Indexes, IndexesIter>(indexes: Indexes) -> Self
    where
        EmbeddedIndex: 'a,
        Indexes: IntoIterator<Item = &'a UnversionedByIdIndex<EmbeddedIndex>, IntoIter = IndexesIter>
            + ExactSizeIterator,
        IndexesIter:
            Iterator<Item = &'a UnversionedByIdIndex<EmbeddedIndex>> + ExactSizeIterator + Clone,
    {
        reduce(indexes)
    }

    fn rereduce<'a, ReducedIndexes, ReducedIndexesIter>(values: ReducedIndexes) -> Self
    where
        Self: 'a,
        ReducedIndexes:
            IntoIterator<Item = &'a Self, IntoIter = ReducedIndexesIter> + ExactSizeIterator,
        ReducedIndexesIter: Iterator<Item = &'a Self> + ExactSizeIterator + Clone,
    {
        rereduce(values)
    }
}

fn reduce<'a, EmbeddedIndex, EmbeddedStats, Id, Indexes, IndexesIter>(
    values: Indexes,
) -> ByIdStats<EmbeddedStats>
where
    Id: IdIndex<EmbeddedIndex> + 'a,
    EmbeddedIndex: 'a,
    Indexes: IntoIterator<Item = &'a Id, IntoIter = IndexesIter> + ExactSizeIterator,
    IndexesIter: Iterator<Item = &'a Id> + ExactSizeIterator + Clone,
    EmbeddedStats: Reducer<EmbeddedIndex>,
{
    let values = values.into_iter();
    let (alive_keys, deleted_keys, total_indexed_bytes) = values
        .clone()
        .map(|index| {
            if index.position() > 0 {
                // Alive key
                (1, 0, u64::from(index.value_size()))
            } else {
                // Deleted
                (0, 1, 0)
            }
        })
        .reduce(
            |(total_alive, total_deleted, total_size), (alive, deleted, size)| {
                (
                    total_alive + alive,
                    total_deleted + deleted,
                    total_size + size,
                )
            },
        )
        .unwrap_or_default();
    ByIdStats {
        alive_keys,
        deleted_keys,
        total_indexed_bytes,
        embedded: EmbeddedStats::reduce(values.map(IdIndex::embedded)),
    }
}

pub trait IdIndex<EmbeddedIndex> {
    fn value_size(&self) -> u32;
    fn position(&self) -> u64;
    fn embedded(&self) -> &EmbeddedIndex;
}

impl<EmbeddedIndex> IdIndex<EmbeddedIndex> for UnversionedByIdIndex<EmbeddedIndex>
where
    EmbeddedIndex: super::EmbeddedIndex,
{
    fn value_size(&self) -> u32 {
        self.value_length
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn embedded(&self) -> &EmbeddedIndex {
        &self.embedded
    }
}

impl<EmbeddedIndex> IdIndex<EmbeddedIndex> for VersionedByIdIndex<EmbeddedIndex>
where
    EmbeddedIndex: super::EmbeddedIndex,
{
    fn value_size(&self) -> u32 {
        self.value_length
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn embedded(&self) -> &EmbeddedIndex {
        &self.embedded
    }
}

fn rereduce<
    'a,
    ReducedIndexes: IntoIterator<Item = &'a ByIdStats<EmbeddedStats>, IntoIter = ReducedIndexesIter>
        + ExactSizeIterator,
    ReducedIndexesIter: Iterator<Item = &'a ByIdStats<EmbeddedStats>> + ExactSizeIterator + Clone,
    EmbeddedStats: Reducer<EmbeddedIndex> + 'a,
    EmbeddedIndex,
>(
    values: ReducedIndexes,
) -> ByIdStats<EmbeddedStats> {
    let values = values.into_iter();
    ByIdStats {
        alive_keys: values.clone().map(|v| v.alive_keys).sum(),
        deleted_keys: values.clone().map(|v| v.deleted_keys).sum(),
        total_indexed_bytes: values.clone().map(|v| v.total_indexed_bytes).sum(),
        // TODO change this to an iterator
        embedded: EmbeddedStats::rereduce(values.map(|v| &v.embedded)),
    }
}
