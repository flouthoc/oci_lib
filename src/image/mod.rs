mod go_set;

use std::fmt;
use std::str::FromStr;
use chrono::prelude::*;
use std::collections::HashMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::serialize;


#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub architecture: Architecture,
    pub os: OS,
    pub rootfs: RootFS,
    pub created: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub config: Option<ImageConfig>,
    pub history: Option<Vec<History>>,
}

impl ImageConfig {
    pub fn load(path: &str) -> Result<ImageConfig, serialize::SerializeError> {
        serialize::deserialize(path)    
    }
    pub fn save(&self, path: &str) -> Result<(), serialize::SerializeError> {
        serialize::serialize(self, path)    
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ImageConfig {
    pub user: Option<String>,
    pub exposed_ports: go_set::GoSet<Port>,
    pub env: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub volumes: go_set::GoSet<String>,
    pub working_dir: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub stop_signal: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Port {
    Udp {
        port: u16,
    },
    Tcp {
        port: u16,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    #[serde(rename = "386")]
    _386,
    Amd64,
    Arm,
    Arm64,
    Mips,
    Mips64,
    Mips64le,
    Mipsle,
    Ppc64,
    Ppc64le,
    S390x,
    Wasm,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OS {
    Aix,
    Android,
    Darwin,
    Dragonfly,
    Freebsd,
    Illumos,
    Js,
    Linux,
    Netbsd,
    Openbsd,
    Plan9,
    Solaris,
    Windows,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootFS {
    #[serde(rename = "type")]
    pub _type: RootFSType,
    pub diff_ids: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    pub created: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub created_by: Option<String>,
    pub comment: Option<String>,
    pub empty_layer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RootFSType {
    Layers,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Udp { port } => write!(f, "{}/udp", port),
            Self::Tcp { port } => write!(f, "{}/tcp", port),
        }
    }
}

impl FromStr for Port {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slash_sp = s.split('/');

        let port = slash_sp
            .next()
            .ok_or(())?
            .parse::<u16>()
            .map_err(|_e| ())?;

        match (slash_sp.next(), slash_sp.next()) {
            (Some("udp"), None) => Ok(Self::Udp { port }),
            (Some("tcp"), None) | (None, None) => Ok(Self::Tcp { port }),
            _ => Err(()),
        }
    }
}


#[cfg(feature = "serde")]
impl serde::Serialize for Port {
	fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
		ser.collect_str(self)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Port {
	fn deserialize<D: serde::Deserializer<'de>>(deser: D) -> Result<Self, D::Error> {
				let s = String::deserialize(deser)?;
				Ok(s.parse::<Port>().unwrap())
	}
}
