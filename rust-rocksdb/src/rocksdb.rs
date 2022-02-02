// Copyright 2014 Tyler Neely
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crocksdb_ffi::{
    self, DBBackupEngine, DBCFHandle, DBCache, DBCompressionType, DBEnv, DBInstance, DBMapProperty,
    DBPinnableSlice, DBSequentialFile, DBStatisticsHistogramType, DBStatisticsTickerType,
    DBTablePropertiesCollection, DBTitanDBOptions, DBWriteBatch,
};

pub struct CFHandle {
    inner: *mut DBCFHandle,
}

impl CFHandle {
    pub fn id(&self) -> u32 {
        unsafe { crocksdb_ffi::crocksdb_column_family_handle_id(self.inner) }
    }
}

impl Drop for CFHandle {
    fn drop(&mut self) {
        unsafe {
            crocksdb_ffi::crocksdb_column_family_handle_destroy(self.inner);
        }
    }
}

fn ensure_default_cf_exists<'a>(
    list: &mut Vec<ColumnFamilyDescriptor<'a>>,
    ttls: &mut Vec<i32>,
    is_titan: bool,
) {
    let contains = list.iter().any(|ref cf| cf.is_default());
    if !contains {
        let mut desc = ColumnFamilyDescriptor::default();
        if is_titan {
            desc.options.set_titandb_options(&TitanDBOptions::new());
        }
        list.push(desc);
        if ttls.len() > 0 {
            ttls.push(0);
        }
    }
}