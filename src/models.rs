use crate::common::*;
use crate::schema::resource;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, Queryable, Selectable, Debug)]
#[diesel(table_name = resource)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Resource {
    pub id: i32,
    pub rid: String,
    pub rname: String,
    pub create_time: Option<NaiveDateTime>,
    pub access_time: Option<NaiveDateTime>,
    pub rpath: String,
    pub rtype: Option<String>,
    pub fpath: Option<String>,
    pub last_modified_time: Option<NaiveDateTime>,
    pub etag: Option<String>,
    pub expires: Option<NaiveDateTime>,
}

#[derive(Clone)]
#[derive(Insertable)]
#[diesel(table_name = resource)]
pub struct NewResource<'a> {
    pub rid: String,
    pub rname: &'a str,
    pub rpath: &'a str,
    pub rtype: Option<&'a str>,
    pub fpath: Option<&'a str>,
    pub etag: Option<&'a str>,
    pub expires: Option<NaiveDateTime>,
}

impl<'a> NewResource<'a> {
    pub fn new(
        rname: &'a str,
        rpath: &'a str,
    ) -> Self {
        NewResource {
            rid: generate_id(),
            rname,
            rpath,
            rtype: None,
            fpath: None,
            etag: None,
            expires: None,
        }
    }

    pub fn set_fpath(&mut self, fpath: &'a str) -> &mut Self {
        self.fpath = Some(fpath);
        self
    }
    pub fn set_etag(&mut self, etag: &'a str) -> &mut Self {
        self.etag = Some(etag);
        self
    }
    pub fn set_expires(&mut self, expires: NaiveDateTime) -> &mut Self {
        self.expires = Some(expires);
        self
    }
    pub fn set_rtype(&mut self, rtype: &'a str) -> &mut Self {
        self.rtype = Some(rtype);
        self
    }
}
