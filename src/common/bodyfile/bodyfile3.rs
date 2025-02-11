use duplicate::duplicate_item;
use getset::{Getters, Setters};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

///
/// This struct implements the bodyfile format generated by TSK 3.x
///
#[derive(Debug, Getters, Setters)]
#[getset(get = "pub with_prefix", set = "pub")]
pub struct Bodyfile3Line {
    md5: String,
    name: String,
    inode: String,
    mode_as_string: String,
    uid: u64,
    gid: u64,
    size: u64,
    atime: i64,
    mtime: i64,
    ctime: i64,
    crtime: i64,
}

impl Default for Bodyfile3Line {
    fn default() -> Self {
        Self::new()
    }
}

impl Bodyfile3Line {
    /// create a new empty bodyfile line
    ///
    /// # Example
    /// ```
    /// use dfir_toolkit::common::bodyfile::Bodyfile3Line;
    ///
    /// let bf = Bodyfile3Line::new();
    /// assert_eq!(bf.get_md5(), "0");
    /// assert_eq!(bf.get_name(), "");
    /// assert_eq!(bf.get_inode(), "0");
    /// assert_eq!(bf.get_mode_as_string(), "");
    /// assert_eq!(*bf.get_uid(), 0);
    /// assert_eq!(*bf.get_gid(), 0);
    /// assert_eq!(*bf.get_size(), 0);
    /// assert_eq!(*bf.get_atime(), -1);
    /// assert_eq!(*bf.get_mtime(), -1);
    /// assert_eq!(*bf.get_ctime(), -1);
    /// assert_eq!(*bf.get_crtime(), -1);
    /// ```
    pub fn new() -> Self {
        Self {
            md5: "0".to_owned(),
            name: "".to_owned(),
            inode: "0".to_owned(),
            mode_as_string: "".to_owned(),
            uid: 0,
            gid: 0,
            size: 0,
            atime: -1,
            mtime: -1,
            ctime: -1,
            crtime: -1,
        }
    }

    #[duplicate_item(
        method_name attribute_name;
        [with_md5]    [md5];
        [with_name]   [name];
        [with_inode]  [inode];
        [with_mode]   [mode_as_string];
    )]
    pub fn method_name(mut self, attribute_name: &str) -> Self {
        self.attribute_name = attribute_name.to_owned();
        self
    }

    #[duplicate_item(
        method_name attribute_name attribute_type;
        [with_owned_md5]    [md5]            [String];
        [with_owned_name]   [name]           [String];
        [with_owned_inode]  [inode]          [String];
        [with_owned_mode]   [mode_as_string] [String];
        [with_uid]    [uid]            [u64];
        [with_gid]    [gid]            [u64];
        [with_size]   [size]           [u64];
        [with_atime]  [atime]          [i64];
        [with_mtime]  [mtime]          [i64];
        [with_ctime]  [ctime]          [i64];
        [with_crtime] [crtime]         [i64];
    )]
    pub fn method_name(mut self, attribute_name: attribute_type) -> Self {
        self.attribute_name = attribute_name;
        self
    }
}

impl fmt::Display for Bodyfile3Line {
    /// exports the line to the format parsable by, eg. `mactime`
    ///
    /// # Example
    /// ```
    /// use dfir_toolkit::common::bodyfile::Bodyfile3Line;
    ///
    /// let bf = Bodyfile3Line::new()
    ///             .with_md5("4bad420da66571dac7f1ace995cc55c6")
    ///             .with_name("sample.txt")
    ///             .with_inode("87915-128-1")
    ///             .with_mode("r/rrwxrwxrwx")
    ///             .with_uid(1003)
    ///             .with_gid(500)
    ///             .with_size(126378)
    ///             .with_atime(12341)
    ///             .with_mtime(12342)
    ///             .with_ctime(12343)
    ///             .with_crtime(12344);
    /// let line = bf.to_string();
    /// assert_eq!(line, "4bad420da66571dac7f1ace995cc55c6|sample.txt|87915-128-1|r/rrwxrwxrwx|1003|500|126378|12341|12342|12343|12344")
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            self.md5,
            self.name,
            self.inode,
            self.mode_as_string,
            self.uid,
            self.gid,
            self.size,
            self.atime,
            self.mtime,
            self.ctime,
            self.crtime
        )
    }
}

#[derive(Debug)]
pub enum Bodyfile3ParserError {
    /// indicates that number of columns is not valid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from(""), Err(Bodyfile3ParserError::WrongNumberOfColumns));
    /// assert_matches!(Bodyfile3Line::try_from("|||||||||"), Err(Bodyfile3ParserError::WrongNumberOfColumns));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|-1|-1|-1"), Ok(_));
    /// assert_matches!(Bodyfile3Line::try_from("0|{\"activity_id\":null,\"channel_name\":\"Microsoft-Windows-PowerShell/Operational\",\"custom_data\":{\"EventData\":{\"ContextInfo\":\"        Severity = Warning\\r\\n        Host Name = ConsoleHost\\r\\n        Host Version = 4.0\\r\\n        Host ID = 5635c559-63c7-4bdc-8bb6-e2aa0448e7b9\\r\\n        Host Application = powershell get-VMNetworkAdapter -ManagementOS | fl | out-file -encoding ASCII VMNetworkAdapterInstances.txt\\r\\n        Engine Version = 4.0\\r\\n        Runspace ID = d315d83a-8923-4530-9553-e63551c33cbc\\r\\n        Pipeline ID = 1\\r\\n        Command Name = \\r\\n        Command Type = Script\\r\\n        Script Name = \\r\\n        Command Path = \\r\\n        Sequence Number = 15\\r\\n        User = TEST\\\\SYSTEM\\r\\n        Shell ID = Microsoft.PowerShell\\r\\n\",\"Payload\":\"Error Message = Could not load file or assembly 'System.Data, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b77a5c561934e089' or one of its dependencies. The media is write protected. (Exception from HRESULT: 0x80070013)\\r\\nFully Qualified Error ID = System.IO.FileLoadException\\r\\n\",\"UserData\":\"\"}},\"event_id\":4100,\"event_record_id\":2424468,\"provider_name\":\"Microsoft-Windows-PowerShell\"}|0||0|0|0|-1|1645178371|-1|-1"), Ok(_));
    /// ```
    WrongNumberOfColumns,

    /// indicates that the uid is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||X|0|0|-1|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalUid));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||-1|0|0|-1|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalUid));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|0|0|-1|-1|-1|-1").unwrap();
    /// assert_eq!(*valid_bf.get_uid(), 1);
    /// ```
    IllegalUid,

    /// indicates that the gid is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|X|0|-1|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalGid));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|-2|0|-1|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalGid));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|2|0|-1|-1|-1|-1").unwrap();
    /// assert_eq!(*valid_bf.get_gid(), 2);
    /// ```
    IllegalGid,

    /// indicates that the size is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|X|-1|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalSize));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|-4|-1|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalSize));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|0|4|-1|-1|-1|-1").unwrap();
    /// assert_eq!(*valid_bf.get_size(), 4);
    /// ```
    IllegalSize,

    /// indicates that the atime is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|X|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalATime));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-5|-1|-1|-1"), Err(Bodyfile3ParserError::IllegalATime));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|0|0|5|-1|-1|-1").unwrap();
    /// assert_eq!(*valid_bf.get_atime(), 5);
    /// ```
    IllegalATime,

    /// indicates that the mtime is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|X|-1|-1"), Err(Bodyfile3ParserError::IllegalMTime));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|-5|-1|-1"), Err(Bodyfile3ParserError::IllegalMTime));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|0|0|-1|5|-1|-1").unwrap();
    /// assert_eq!(*valid_bf.get_mtime(), 5);
    /// ```
    IllegalMTime,

    /// indicates that the ctime is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|-1|X|-1"), Err(Bodyfile3ParserError::IllegalCTime));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|-1|-5|-1"), Err(Bodyfile3ParserError::IllegalCTime));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|0|0|-1|-1|5|-1").unwrap();
    /// assert_eq!(*valid_bf.get_ctime(), 5);
    /// ```
    IllegalCTime,

    /// indicates that the crtime is syntactically invalid
    ///
    /// # Examples
    /// ```
    /// extern crate matches;
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// use matches::assert_matches;
    ///
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|-1|-1|X"), Err(Bodyfile3ParserError::IllegalCRTime));
    /// assert_matches!(Bodyfile3Line::try_from("0||0||0|0|0|-1|-1|-1|-5"), Err(Bodyfile3ParserError::IllegalCRTime));
    /// let valid_bf = Bodyfile3Line::try_from("0||0||1|0|0|-1|-1|-1|5").unwrap();
    /// assert_eq!(*valid_bf.get_crtime(), 5);
    /// ```
    IllegalCRTime,
}

/// implements `Display` for this enum
///
/// # Example
/// ```
/// use dfir_toolkit::common::bodyfile::Bodyfile3ParserError;
///
/// let myerror = Bodyfile3ParserError::IllegalCRTime;
/// assert_eq!(myerror.to_string(), "IllegalCRTime")
/// ```
impl fmt::Display for Bodyfile3ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Bodyfile3ParserError {}

impl TryFrom<&str> for Bodyfile3Line {
    type Error = Bodyfile3ParserError;

    /// parses a bodyfile line
    ///
    /// # Example
    /// ```
    /// use dfir_toolkit::common::bodyfile::{Bodyfile3Line, Bodyfile3ParserError};
    /// use std::convert::TryFrom;
    /// let bf_line = Bodyfile3Line::try_from("0|ls -l |wc|1|2|3|4|5|6|7|8|9").unwrap();
    /// assert_eq!(bf_line.get_md5(), "0");
    /// assert_eq!(bf_line.get_name(), "ls -l |wc");
    /// assert_eq!(bf_line.get_inode(), "1");
    /// assert_eq!(bf_line.get_mode_as_string(), "2");
    /// assert_eq!(*bf_line.get_uid(), 3);
    /// assert_eq!(*bf_line.get_gid(), 4);
    /// assert_eq!(*bf_line.get_size(), 5);
    /// assert_eq!(*bf_line.get_atime(), 6);
    /// assert_eq!(*bf_line.get_mtime(), 7);
    /// assert_eq!(*bf_line.get_ctime(), 8);
    /// assert_eq!(*bf_line.get_crtime(), 9);
    /// ```

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 11 {
            return Err(Self::Error::WrongNumberOfColumns);
        }

        let name_chunks = parts.len() - 10;
        let md5 = parts[0];
        let name = parts[1..name_chunks + 1].join("|");
        let inode = parts[2 + name_chunks - 1];
        let mode = parts[3 + name_chunks - 1];
        let uid = str::parse::<u64>(parts[4 + name_chunks - 1]).or(Err(Self::Error::IllegalUid))?;
        let gid = str::parse::<u64>(parts[5 + name_chunks - 1]).or(Err(Self::Error::IllegalGid))?;

        let size =
            str::parse::<u64>(parts[6 + name_chunks - 1]).or(Err(Self::Error::IllegalSize))?;
        let atime =
            str::parse::<i64>(parts[7 + name_chunks - 1]).or(Err(Self::Error::IllegalATime))?;
        if atime < -1 {
            return Err(Self::Error::IllegalATime);
        }
        let mtime =
            str::parse::<i64>(parts[8 + name_chunks - 1]).or(Err(Self::Error::IllegalMTime))?;
        if mtime < -1 {
            return Err(Self::Error::IllegalMTime);
        }
        let ctime =
            str::parse::<i64>(parts[9 + name_chunks - 1]).or(Err(Self::Error::IllegalCTime))?;
        if ctime < -1 {
            return Err(Self::Error::IllegalCTime);
        }
        let crtime =
            str::parse::<i64>(parts[10 + name_chunks - 1]).or(Err(Self::Error::IllegalCRTime))?;
        if crtime < -1 {
            return Err(Self::Error::IllegalCRTime);
        }
        Ok(Self {
            md5: md5.to_owned(),
            name: name.to_owned(),
            inode: inode.to_owned(),
            mode_as_string: mode.to_owned(),
            uid,
            gid,
            size,
            atime,
            mtime,
            ctime,
            crtime,
        })
    }
}
