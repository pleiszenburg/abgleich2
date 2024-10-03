use crate::datasettype::DatasetType;
use crate::property::Property;
use crate::rawproperty::RawProperty;
use crate::stat::Stat;

pub struct Meta {

    // name or ZFS path
    pub name: String,

    // Properties which can be configured
    pub atime: Property<bool>,
    pub canmount: Property<bool>,
    pub checksum: Property<bool>,
    pub compression: Property<String>,
    pub dedup: Property<bool>,
    pub encryption: Property<bool>,
    pub filesystem_count: Property<u64>,
    pub filesystem_limit: Property<u64>,
    pub mountpoint: Property<String>,
    pub readonly: Property<bool>,
    pub redundant_metadata: Property<String>,
    pub relatime: Property<bool>,
    pub sharenfs: Property<bool>,
    pub snapshot_count: Property<u64>,
    pub snapshot_limit: Property<u64>,
    pub sync: Property<String>,
    pub volmode: Property<String>,

    // Properties which serve as statistics / immutable
    pub available: Stat<u64>,
    pub compressratio: Stat<f32>,
    pub creation: Stat<u64>,
    pub datasettype: Stat<DatasetType>,
    pub guid: Stat<u64>,
    pub logicalreferenced: Stat<u64>,
    pub logicalused: Stat<u64>,
    pub mounted: Stat<bool>,
    pub refcompressratio: Stat<f32>,
    pub referenced: Stat<u64>,
    pub used: Stat<u64>,
    pub usedbychildren: Stat<u64>,
    pub usedbydataset: Stat<u64>,
    pub usedbyrefreservation: Stat<u64>,
    pub usedbysnapshots: Stat<u64>,
    pub version: Stat<u64>,
    pub written: Stat<u64>,

}

impl Meta {

    pub fn new (name: &str) -> Self {

        Self {

            name: name.to_string(),

            atime: Property::from_empty(),
            canmount: Property::from_empty(),
            checksum: Property::from_empty(),
            compression: Property::from_empty(),
            dedup: Property::from_empty(),
            encryption: Property::from_empty(),
            filesystem_count: Property::from_empty(),
            filesystem_limit: Property::from_empty(),
            mountpoint: Property::from_empty(),
            readonly: Property::from_empty(),
            redundant_metadata: Property::from_empty(),
            relatime: Property::from_empty(),
            sharenfs: Property::from_empty(),
            snapshot_count: Property::from_empty(),
            snapshot_limit: Property::from_empty(),
            sync: Property::from_empty(),
            volmode: Property::from_empty(),

            available: Stat::from_empty(),
            compressratio: Stat::from_empty(),
            creation: Stat::from_empty(),
            datasettype: Stat::from_empty(),
            guid: Stat::from_empty(),
            logicalreferenced: Stat::from_empty(),
            logicalused: Stat::from_empty(),
            mounted: Stat::from_empty(),
            refcompressratio: Stat::from_empty(),
            referenced: Stat::from_empty(),
            used: Stat::from_empty(),
            usedbychildren: Stat::from_empty(),
            usedbydataset: Stat::from_empty(),
            usedbyrefreservation: Stat::from_empty(),
            usedbysnapshots: Stat::from_empty(),
            version: Stat::from_empty(),
            written: Stat::from_empty(),

        }

    }

    pub fn fill(&mut self, raw_property: &RawProperty) {

        match raw_property.name.as_str() {

            "atime" => { self.atime.fill(raw_property) },
            "canmount" => { self.canmount.fill(raw_property) },
            "checksum" => { self.checksum.fill(raw_property) },
            "compression" => { self.compression.fill(raw_property) },
            "type" => { self.datasettype.fill(raw_property) },
            "dedup" => { self.dedup.fill(raw_property) },
            "encryption" => { self.encryption.fill(raw_property) },
            "filesystem_count" => { self.filesystem_count.fill(raw_property) },
            "filesystem_limit" => { self.filesystem_limit.fill(raw_property) },
            "mountpoint" => { self.mountpoint.fill(raw_property) },
            "readonly" => { self.readonly.fill(raw_property) },
            "redundant_metadata" => { self.redundant_metadata.fill(raw_property) },
            "relatime" => { self.relatime.fill(raw_property) },
            "sharenfs" => { self.sharenfs.fill(raw_property) },
            "snapshot_count" => { self.snapshot_count.fill(raw_property) },
            "snapshot_limit" => { self.snapshot_limit.fill(raw_property) },
            "sync" => { self.sync.fill(raw_property) },
            "volmode" => { self.volmode.fill(raw_property) },

            "available" => { self.available.fill(raw_property) },
            "compressratio" => { self.compressratio.fill(raw_property) },
            "creation" => { self.creation.fill(raw_property) },
            "guid" => { self.guid.fill(raw_property) },
            "logicalreferenced" => { self.logicalreferenced.fill(raw_property) },
            "logicalused" => { self.logicalused.fill(raw_property) },
            "mounted" => { self.mounted.fill(raw_property) },
            "refcompressratio" => { self.refcompressratio.fill(raw_property) },
            "referenced" => { self.referenced.fill(raw_property) },
            "used" => { self.used.fill(raw_property) },
            "usedbychildren" => { self.usedbychildren.fill(raw_property) },
            "usedbydataset" => { self.usedbydataset.fill(raw_property) },
            "usedbyrefreservation" => { self.usedbyrefreservation.fill(raw_property) },
            "usedbysnapshots" => { self.usedbysnapshots.fill(raw_property) },
            "version" => { self.version.fill(raw_property) },
            "written" => { self.written.fill(raw_property) },

            _ => { /* TODO unknown parameter */ }

        }

    }

}
